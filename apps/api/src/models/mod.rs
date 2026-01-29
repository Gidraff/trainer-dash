use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Deserialize)]
pub struct FeedbackRequest {
    pub feedback: String,
}
