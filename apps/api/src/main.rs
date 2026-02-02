use axum::http::{Method, header};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

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
    // 1. Load environment variables
    dotenvy::dotenv().ok();

    // 2. Initialize Logging (MOVED TO TOP)
    // This ensures every step below is logged properly.
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    tracing::info!("Initializing FitFlow API...");

    // 3. Database Setup
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Using a loop to wait for the DB (handles the "Sidecar Race Condition")
    let pool = loop {
        tracing::info!("Connecting to database at 127.0.0.1:5432...");
        match sqlx::PgPool::connect(&database_url).await {
            Ok(p) => {
                tracing::info!("✅ Database connection established");
                break p;
            },
            Err(e) => {
                tracing::warn!("Waiting for DB... (Error: {}). Retrying in 2s...", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // 4. Keycloak Setup
    let issuer_url = env::var("KEYCLOAK_ISSUER_URL")
        .expect("KEYCLOAK_ISSUER_URL must be set");

    let internal_url = env::var("KEYCLOAK_INTERNAL_URL")
        .unwrap_or_else(|_| "http://keycloak:8080/auth/realms/trainer-app".to_string());

    let jwks_url = format!("{}/protocol/openid-connect/certs", internal_url);
    tracing::info!("🔐 Fetching JWKS from: {}", jwks_url);

    let response = reqwest::get(&jwks_url)
        .await
        .expect("CRITICAL: Failed to connect to Keycloak.");

    let body_text = response
        .text()
        .await
        .expect("Failed to read Keycloak response body");

    if body_text.contains("<html>") || body_text.contains("Page not found") {
        tracing::error!("Keycloak returned HTML instead of JSON. Check realm configuration.");
        panic!("Keycloak config error. See logs.");
    }

    let jwks: auth::jwks::Jwks = serde_json::from_str(&body_text)
        .expect("Failed to parse JWKS JSON.");

    let auth_config = auth::middleware::AuthState {
        jwks: Arc::new(jwks),
        issuer: issuer_url,
        audience: "trainer-api".into(),
    };

    let state = AppState {
        db: pool.clone(),
        auth: auth_config.clone(),
    };

    // 5. Routes & Middleware
    let protected_routes: Router<AppState> = Router::new()
        .route("/trainer/secure", get(|| async { "Secure content" }))
        // ... (your other routes)
        .layer(axum::middleware::from_fn_with_state(
            auth_config,
            auth::middleware::auth_middleware,
        ));

    let public_routes: Router<AppState> = Router::new()
        .route("/", get(|| async { "FitFlow AI API - v1.0" }))
        .route("/health", get(|| async { "OK" }));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .with_state(state);

    // 6. Networking & Server Start
    let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid u16");

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Failed to parse socket address");

    tracing::info!("🚀 Server starting at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}