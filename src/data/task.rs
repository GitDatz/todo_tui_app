use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: usize,
    name: String,
    description: String,
    date_added: DateTime<Utc>
}
