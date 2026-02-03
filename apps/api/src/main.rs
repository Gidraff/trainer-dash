use axum::http::{Method, header};
use axum::{Router, routing::{get, post, put, delete}};
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
    // 1. IMMEDIATE BINDING
    // We bind early so Kubernetes probes (port 8080) don't get "Connection Refused"
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("CRITICAL: Failed to bind to 0.0.0.0:8080");

    // FORCE STDOUT FLUSH
    use std::io::Write;
    let _ = std::io::stdout().flush();
    
    eprintln!("=== FITFLOW API STARTING (Port 8080 Active) ===");
    
    dotenvy::dotenv().ok();
    
    // Check env vars
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("❌ DATABASE_URL missing!");
        std::process::exit(1);
    });
    
    let internal_url = env::var("KEYCLOAK_INTERNAL_URL").unwrap_or_else(|_| {
        eprintln!("❌ KEYCLOAK_INTERNAL_URL missing!");
        std::process::exit(1);
    });
    
    let issuer_url = env::var("KEYCLOAK_ISSUER_URL")
        .unwrap_or_else(|_| internal_url.clone());
    
    // 2. DATABASE CONNECT WITH RETRY
    eprintln!("Connecting to database...");
    let pool = loop {
        match sqlx::PgPool::connect(&database_url).await {
            Ok(p) => {
                eprintln!("✅ Database connected!");
                break p;
            },
            Err(e) => {
                eprintln!("DB error: {}. Retrying in 2s...", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };
    
    eprintln!("Running migrations...");
    sqlx::migrate!().run(&pool).await
        .expect("Migrations failed");
    eprintln!("✅ Migrations complete");
    
    // 3. KEYCLOAK JWKS FETCH
    let jwks_url = format!("{}/protocol/openid-connect/certs", internal_url);
    eprintln!("Fetching JWKS from: {}", jwks_url);
    
    let body_text = reqwest::get(&jwks_url).await
        .expect("Failed to connect to Keycloak")
        .text().await
        .expect("Failed to read response");
    
    if body_text.contains("<html>") {
        eprintln!("❌ Keycloak returned HTML! (Check your INTERNAL_URL)");
        std::process::exit(1);
    }
    
    let jwks: auth::jwks::Jwks = serde_json::from_str(&body_text)
        .expect("Failed to parse JWKS");
    eprintln!("✅ JWKS loaded");
    
    // 4. APP STATE & ROUTES
    let auth_config = auth::middleware::AuthState {
        jwks: Arc::new(jwks),
        issuer: issuer_url,
        audience: "trainer-api".into(),
    };
    
    let state = AppState {
        db: pool.clone(),
        auth: auth_config.clone(),
    };
    
    let public_routes = Router::new()
        .route("/", get(|| async { "FitFlow API v1.0" }))
        .route("/health", get(|| async { "OK" }));

    let protected_routes = Router::new()
        .route("/trainer/secure", get(|| async { "Secure" }))
        .route("/trainer/clients", post(handlers::trainer::create_client))
        .route("/trainer/clients", get(handlers::trainer::list_clients))
        .route("/trainer/clients/:id", get(handlers::trainer::get_client_by_id))
        .route("/trainer/clients/:id", put(handlers::trainer::update_client))
        .route("/trainer/clients/:id", delete(handlers::trainer::delete_client))
        .route("/trainer/sessions/:client_id", get(handlers::trainer::get_client_sessions))
        .route("/trainer/sessions/:client_id", post(handlers::trainer::log_workout_session))
        .route("/trainer/sessions/:id/feedback", post(handlers::trainer::add_session_feedback))
        .layer(axum::middleware::from_fn_with_state(
            auth_config,
            auth::middleware::auth_middleware,
        ));
    
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
    
    eprintln!("✅ Server ready! Finalizing serve...");
    axum::serve(listener, app).await.unwrap();
}