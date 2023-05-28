use std::sync::Arc;

use crate::{util::ValidatedQuery, AppState};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Extension};

use super::util::{ControlParamsDeviceId, ControlParams};

/// Json Serialized based API routes,
/// Much wider accepted method of sending responses
pub fn routes() -> Router {
    Router::new()
        .route("/randint32", get(randint32))
        .route("/randuniform", get(randuniform))
        .route("/randnormal", get(randnormal))
        .route("/randhex", get(randhex))
        .route("/randbase64", get(randbase64))
        .route("/randbytes", get(randbytes))
        .route("/devices", get(devices))
}

/// /devices
async fn devices(Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let handle = state.meterfeeder_handle.lock().await;
    Ok(Json(handle.list_generators()))
}

/// /randint32
async fn randint32(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_int_32(&device_id) {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// /randuniform
async fn randuniform(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_uniform(&device_id) {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randnormal
async fn randnormal(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_normal(&device_id) {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// /randhex
async fn randhex(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(Json(hex::encode(buff))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randbase64
async fn randbase64(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(Json(base64::encode(buff))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randbase64
async fn randbytes(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(Json(buff)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}