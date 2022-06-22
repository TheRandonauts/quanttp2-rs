use axum::{response::IntoResponse, http::StatusCode, Router, routing::get, Json};
use serde_json::json;
use crate::util::ValidatedQuery;

use super::util::{ControlParamsDeviceId, SERVER_NAME};

pub fn routes() -> Router {
    Router::new()
    .route("/clear", get(clear))
    .route("/reset", get(reset))
    .route("/status", get(status))
}

/// /status
async fn status() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({"server": SERVER_NAME, "status": true, "devices": ["༼ つ ◕_◕ ༽つ", "/ᐠ｡ꞈ｡ᐟ\\"]})))
}

/// /reset
async fn reset() -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::from_u16(204).unwrap())
}


/// /clear
async fn clear(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    Ok(StatusCode::from_u16(204).unwrap())

}
