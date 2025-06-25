use std::collections::BTreeMap;
use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use crate::task::Task;
use crate::db::DB;

#[derive(Serialize, Deserialize)]
struct JSONResponse {
    msg: String,
    result: serde_json::Value
}

async fn create(
    Json(task): Json<Task>,
    Extension(db): Extension<Arc<DB>>
) -> (StatusCode, Json<JSONResponse>) {
    match db.store(&task).await {
        Ok(()) => (StatusCode::CREATED, Json(JSONResponse {
            msg: "Created task".to_owned(),
            result: serde_json::to_value(task).unwrap()
        })),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(JSONResponse {
            msg: "Failed to create task".to_owned(),
            result: serde_json::to_value(e.to_string()).unwrap()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_create() {
        let db = Arc::new(DB::new(true).await.unwrap());
        let task: Task = serde_json::from_str("{ \"title\": \"foo\", \"status\": \"bar\", \"due\": 0 }").unwrap();
        let resp = create(
            Json(task.clone()),
            Extension(db.clone())
        ).await;
        assert_eq!(resp.0, StatusCode::CREATED);
        assert!({
            let Json(j) = resp.1;
            if j.result == serde_json::to_value(task).unwrap() {
                true
            } else {
                false
            }
        });
    }
}
