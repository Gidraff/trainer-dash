use crate::AppState;
use crate::auth::claims::Claims;
use crate::models::{Client, CreateClientRequest, Session, TrainerFeedbackRequest};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;

// --- Client Handlers ---

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(client))
}

pub async fn list_clients(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Client>>, StatusCode> {
    let clients = sqlx::query_as::<_, Client>(
        "SELECT id, name, goal, profile FROM clients WHERE trainer_id = $1",
    )
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(clients))
}

pub async fn update_client(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClientRequest>,
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as::<_, Client>(
        "UPDATE clients SET name = $1, goal = $2, profile = $3 
         WHERE id = $4 AND trainer_id = $5 
         RETURNING id, name, goal, profile",
    )
    .bind(payload.name)
    .bind(payload.goal)
    .bind(payload.profile)
    .bind(client_id)
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(client))
}

pub async fn delete_client(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM clients WHERE id = $1 AND trainer_id = $2")
        .bind(client_id)
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 { return Err(StatusCode::NOT_FOUND); }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_client_by_id(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Client>, StatusCode> {
    let client = sqlx::query_as::<_, Client>(
        "SELECT id, name, goal, profile FROM clients WHERE id = $1 AND trainer_id = $2",
    )
    .bind(client_id)
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(client))
}

// --- Session Handlers ---

pub async fn get_client_sessions(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Session>>, StatusCode> {
    let sessions = sqlx::query_as!(
        Session,
        r#"
        SELECT 
            id, 
            client_id as "client_id!", 
            workout_id as "workout_id?", 
            date, 
            weight as "weight: Decimal", 
            mood as "mood?", 
            energy_level as "energy_level?", 
            athlete_rating as "athlete_rating?", 
            athlete_notes as "athlete_notes?", 
            trainer_feedback as "trainer_feedback?", 
            performance_rating as "performance_rating?", 
            created_at as "created_at?"
        FROM sessions 
        WHERE client_id = $1 
          AND client_id IN (SELECT id FROM clients WHERE trainer_id = $2)
        ORDER BY date DESC
        "#,
        client_id,
        claims.sub
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(sessions))
}

pub async fn log_workout_session(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<serde_json::Value>, 
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "INSERT INTO sessions (client_id, workout_id, date, weight, mood, energy_level, athlete_rating, athlete_notes) 
         VALUES ($1, $2, CURRENT_DATE, $3, $4, $5, $6, $7)"
    )
    .bind(payload["client_id"].as_str().and_then(|s| uuid::Uuid::parse_str(s).ok()))
    .bind(payload["workout_id"].as_str().and_then(|s| uuid::Uuid::parse_str(s).ok()))
    .bind(payload["weight"].as_f64())
    .bind(payload["mood"].as_str())
    .bind(payload["energy_level"].as_i64())
    .bind(payload["athlete_rating"].as_i64())
    .bind(payload["athlete_notes"].as_str())
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn add_session_feedback(
    State(state): State<AppState>,
    Path(session_id): Path<uuid::Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrainerFeedbackRequest>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "UPDATE sessions SET trainer_feedback = $1, performance_rating = $2 
         WHERE id = $3 AND client_id IN (SELECT id FROM clients WHERE trainer_id = $4)",
        payload.feedback,
        payload.performance_rating,
        session_id,
        claims.sub
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 { return Err(StatusCode::NOT_FOUND); }
    Ok(StatusCode::OK)
}