use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BlogPost {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub body: String,
}

impl BlogPost {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            body: message,
        }
    }
}
