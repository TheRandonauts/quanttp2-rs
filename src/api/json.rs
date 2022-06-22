use crate::util::ValidatedQuery;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use super::util::{ControlParamsDeviceId, ControlParams, SERVER_NAME};

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
async fn devices() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randint32
async fn randint32(
    ValidatedQuery(ControlParamsDeviceId { device_id }): ValidatedQuery<ControlParamsDeviceId>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randuniform
async fn randuniform(
    ValidatedQuery(ControlParamsDeviceId { device_id }): ValidatedQuery<ControlParamsDeviceId>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randnormal
async fn randnormal(
    ValidatedQuery(ControlParams { device_id, length }): ValidatedQuery<ControlParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randhex
async fn randhex(
    ValidatedQuery(ControlParams { device_id, length }): ValidatedQuery<ControlParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randbase64
async fn randbase64(
    ValidatedQuery(ControlParams { device_id, length }): ValidatedQuery<ControlParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}

/// /randbase64
async fn randbytes(
    ValidatedQuery(ControlParams { device_id, length }): ValidatedQuery<ControlParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{device_id:?}, {length:?}");
    Ok(Json(json!({ "server": SERVER_NAME })))
}
