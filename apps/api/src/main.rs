use axum::http::{Method, header};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod auth;
mod db;
mod handlers;
mod models;

// 1. Define a unified AppState to resolve the type mismatch
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: auth::middleware::AuthState,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: PgPool = db::create_pool(&database_url).await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Fetch JWKS for the AuthState
    let jwks_url = "http://localhost:8081/realms/trainer-app/protocol/openid-connect/certs";
    let jwks: auth::jwks::Jwks = reqwest::get(jwks_url).await.unwrap().json().await.unwrap();

    let auth_config = auth::middleware::AuthState {
        jwks: Arc::new(jwks),
        issuer: "http://localhost:8081/realms/trainer-app".into(),
        audience: "trainer-api".into(),
    };

    // 2. Initialize the unified state
    let state = AppState {
        db: pool.clone(),
        auth: auth_config.clone(),
    };

    let protected_routes: Router<AppState> = Router::new()
        .route(
            "/trainer/secure",
            get(|| async { "Secure trainer content" }),
        )
        .route("/trainer/clients", post(handlers::trainer::create_client))
        .route("/trainer/clients", get(handlers::trainer::list_clients))
        .route(
            "/trainer/sessions/:id/feedback",
            post(handlers::trainer::add_session_feedback),
        )
        .layer(axum::middleware::from_fn_with_state(
            auth_config,
            auth::middleware::auth_middleware,
        ));

    // Initialize public routes with the SAME type
    let public_routes: Router<AppState> = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health", get(|| async { "OK" }));

    let cors = CorsLayer::new()
        // Explicitly allow the Vite dev server
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // This is crucial: the browser must be allowed to send the Authorization header
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);
    // Now merging will work perfectly because the types match
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server started successfully on {}", addr);

    // Now into_make_service() will be available because Router state is ()
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
