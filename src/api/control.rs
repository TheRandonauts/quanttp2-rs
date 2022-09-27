use std::{sync::Arc};

use axum::{response::IntoResponse, http::StatusCode, Router, routing::get, Json, Extension};
use serde_json::json;
use crate::{util::ValidatedQuery, AppState};
use gethostname::gethostname;
use super::util::{ControlParamsDeviceId, get_current_time};

pub fn routes() -> Router {
    Router::new()
    .route("/clear", get(clear))
    .route("/reset", get(reset))
    .route("/status", get(status))
}

/// /status
async fn status(Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let uptime = get_current_time() - state.start_ts;
    Ok(Json(json!({"server": gethostname().to_str().unwrap(), "status": true, "uptime": uptime.as_secs(), "version": env!("CARGO_PKG_VERSION")})))
}

/// /reset
async fn reset(Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.reset() {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /clear
async fn clear(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.clear(&device_id) {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
