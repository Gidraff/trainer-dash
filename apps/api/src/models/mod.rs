use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub goal: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Workout {
    pub id: Uuid,
    pub client_id: Uuid,
    pub name: String,
    pub plan: serde_json::Value,
    pub duration_weeks: i32,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub client_id: Uuid,
    pub workout_id: Option<Uuid>,
    pub date: NaiveDate,
    pub weight: Option<Decimal>,
    pub mood: Option<String>,
    pub energy_level: Option<i32>,
    pub athlete_rating: Option<i32>,
    pub athlete_notes: Option<String>,
    pub trainer_feedback: Option<String>,
    pub performance_rating: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct TrainerFeedbackRequest {
    pub feedback: String,
    pub performance_rating: i32,
}
