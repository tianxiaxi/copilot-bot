//! # routers
//!
//! `routers` is the app routers for axum web server

use axum::http::Method;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod health_check;

/// the collect of routers for chat
pub mod chat;

/// return all the routers of the server
pub fn get_app_routers() -> Router {
    let app = Router::new()
        .merge(common_routers())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any));
    app
}

/// common routers without any logic bussiness
fn common_routers() -> Router {
    let routers = Router::new()
        .route("/", get(health_check::welcome))
        .route("/health", get(health_check::check_server_health));
    routers
}
