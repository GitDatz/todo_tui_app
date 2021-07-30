use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub date_added: DateTime<Utc>
}
