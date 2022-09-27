//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PUT /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-todos
//! ```

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    Router, Extension,
};
use std::{
    net::SocketAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::api::api_routes;
use std::process::Command;


use std::sync::Arc;
use libmeterfeeder_rs::meterfeeder::{MeterFeederInstance, MeterFeeder};
struct AppState {
    meterfeeder_handle: tokio::sync::Mutex<MeterFeeder>,
    start_ts: Duration
}

mod api;
mod util;
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "pod-entropy-oxidized=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let reset_logs = Command::new("/home/taha/reset")
        .output()
        .expect("failed to execute process");
    
    println!("Device Reset Status: {}", reset_logs.status.success());

    let start_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    let instance= MeterFeederInstance::new().expect("Failed to create instance");
    let shared_state = Arc::new(AppState { meterfeeder_handle: tokio::sync::Mutex::new(instance), start_ts });

    

    // Compose the routes
    let app = Router::new()
        .nest("/api", api_routes())
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(shared_state))
                .into_inner(),
        );
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into()).parse::<u16>().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("QRNG listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}