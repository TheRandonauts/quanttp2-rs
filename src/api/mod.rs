mod json;
mod control;
mod raw;
mod util;
use axum::{Router};

/// All of the API routes for the project
pub fn api_routes() -> Router {
    Router::new()
    .merge(control::routes())
    .merge(Router::new().nest("/json", json::routes()))
    .merge(raw::routes())
}