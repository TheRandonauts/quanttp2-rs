use std::sync::Arc;

use axum::{response::IntoResponse, http::StatusCode, Router, routing::get, Json, Extension};
use serde_json::json;
use crate::{util::ValidatedQuery, AppState};

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
async fn reset(Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.reset() {
        Ok(x) => Ok(Json(x.to_string())),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /clear
async fn clear(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.clear(&device_id) {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
