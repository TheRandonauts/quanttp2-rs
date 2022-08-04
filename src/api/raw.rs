use std::sync::Arc;

use axum::{response::IntoResponse, http::StatusCode, Router, routing::get, Extension};
use crate::{util::ValidatedQuery, AppState};

use super::util::{ControlParamsDeviceId, ControlParams};

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
    Ok(handle.list_generators().join(","))
}


/// /randint32
async fn randint32(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_int_32(&device_id) {
        Ok(x) => Ok(x.to_string()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// /randuniform
async fn randuniform(ValidatedQuery(ControlParamsDeviceId{device_id}): ValidatedQuery<ControlParamsDeviceId>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_uniform(&device_id) {
        Ok(x) => Ok(x.to_string()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randnormal
async fn randnormal(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.rand_normal(&device_id) {
        Ok(x) => Ok(x.to_string()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// /randhex
async fn randhex(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(format!("{:X?}",buff)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randbase64
async fn randbase64(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(base64::encode(buff)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}


/// /randbase64
async fn randbytes(ValidatedQuery(ControlParams{device_id, length}): ValidatedQuery<ControlParams>, Extension(state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    let mut handle = state.meterfeeder_handle.lock().await;
    match handle.get_bytes(length,&device_id) {
        Ok(buff) => Ok(buff),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}