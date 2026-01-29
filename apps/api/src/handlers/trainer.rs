use crate::AppState;
use crate::auth::claims::Claims;
use crate::models::{Client, CreateClientRequest, FeedbackRequest};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

// POST /trainer/clients
pub async fn create_client(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClientRequest>,
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as::<_, Client>(
        "INSERT INTO clients (trainer_id, name, goal, profile) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, name, goal, profile",
    )
    .bind(&claims.sub)
    .bind(payload.name)
    .bind(payload.goal)
    .bind(payload.profile)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("❌ Database Error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(client))
}

// GET /trainer/clients
pub async fn list_clients(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Client>>, StatusCode> {
    // Access the pool via state.db
    let clients = sqlx::query_as::<_, Client>(
        "SELECT id, name, goal, profile FROM clients WHERE trainer_id = $1",
    )
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("❌ Database Error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(clients))
}

// POST /trainer/sessions/:id/feedback
pub async fn add_session_feedback(
    State(state): State<AppState>, // 👈 Change this from PgPool/Pool<Postgres> to AppState
    Path(session_id): Path<uuid::Uuid>,
    Json(payload): Json<FeedbackRequest>,
) -> Result<StatusCode, StatusCode> {
    // 👈 Use state.db here
    sqlx::query("UPDATE sessions SET feedback = $1 WHERE id = $2")
        .bind(payload.feedback)
        .bind(session_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}
