use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub desc: Option<String>,
    pub status: String,
    pub due: i64
}
