use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    description: String,
    date_added: DateTime<Utc>
}
