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

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: auth::middleware::AuthState,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // 1. Database Setup
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut retry_count = 0;
    let pool = loop {
        // Explicitly handle the Result returned by the async connect call
        let connection_attempt = sqlx::PgPool::connect(&database_url).await;

        match connection_attempt {
            Ok(p) => break p,
            Err(e) => {
                println!("Waiting for DB... {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };

    // Run migrations (this will only work if the API compiles!)
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // 2. Keycloak Setup (Local Configuration)
    // We use 8081 because the API is running on your host machine
    let issuer_url = env::var("KEYCLOAK_ISSUER_URL")
        .expect("KEYCLOAK_ISSUER_URL must be set in .env (e.g., http://localhost:8081/auth/realms/trainer-app)s");

    let internal_url = env::var("KEYCLOAK_INTERNAL_URL")
        .unwrap_or_else(|_| "http://keycloak:8080/auth/realms/trainer-app".to_string());

    let jwks_url = format!("{}/protocol/openid-connect/certs", internal_url);
    println!("🔐 Fetching JWKS from: {}", jwks_url);

    // Fetch the response safely to debug HTML errors
    let response = reqwest::get(jwks_url)
        .await
        .expect("CRITICAL: Failed to connect to Keycloak. Is Docker running?");

    let body_text = response
        .text()
        .await
        .expect("Failed to read Keycloak response body");

    // Check if we got HTML instead of JSON (common when the Realm name is wrong)
    if body_text.contains("<html>") || body_text.contains("Page not found") {
        println!("\n--- ERROR: KEYCLOAK RETURNED HTML ---");
        println!("{}", body_text);
        println!("--------------------------------------\n");
        panic!(
            "Keycloak returned HTML. Check if realm 'trainer-app' exists at http://localhost:8081"
        );
    }

    let jwks: auth::jwks::Jwks = serde_json::from_str(&body_text)
        .expect("Failed to parse JWKS JSON. See the response body above.");

    let auth_config = auth::middleware::AuthState {
        jwks: Arc::new(jwks),
        issuer: issuer_url,
        audience: "trainer-api".into(),
    };

    let state = AppState {
        db: pool.clone(),
        auth: auth_config.clone(),
    };

    // 3. Routes
    let protected_routes: Router<AppState> = Router::new()
        .route(
            "/trainer/secure",
            get(|| async { "Secure trainer content" }),
        )
        .route("/trainer/clients", post(handlers::trainer::create_client))
        .route("/trainer/clients", get(handlers::trainer::list_clients))
        .route(
            "/trainer/clients/:id",
            get(handlers::trainer::get_client_by_id),
        )
        .route(
            "/trainer/clients/:id",
            axum::routing::put(handlers::trainer::update_client),
        )
        .route(
            "/trainer/clients/:id",
            axum::routing::delete(handlers::trainer::delete_client),
        )
        .route(
            "/trainer/sessions/:client_id",
            get(handlers::trainer::get_client_sessions),
        )
        .route(
            "/trainer/sessions/:client_id",
            post(handlers::trainer::log_workout_session),
        )
        .route(
            "/trainer/sessions/:id/feedback",
            post(handlers::trainer::add_session_feedback),
        )
        .layer(axum::middleware::from_fn_with_state(
            auth_config,
            auth::middleware::auth_middleware,
        ));

    let public_routes: Router<AppState> = Router::new()
        .route("/", get(|| async { "FitFlow AI API - v1.0" }))
        .route("/health", get(|| async { "OK" }));

    // 4. CORS
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .with_state(state);

    // 5. Server Start
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n🚀 FitFlow API running locally on http://localhost:8080");
    println!("📡 Connecting to Docker Keycloak on http://localhost:8081");
    println!("🗄️  Connecting to Docker Postgres on localhost:5432\n");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
