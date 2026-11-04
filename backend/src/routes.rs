use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::{AppState}; 



pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        // Дозволити запити з будь-яких адрес
        .allow_origin(Any)
        // Дозволити ці методи
        .allow_methods([Method::GET, Method::POST, Method:: PUT, Method::PATCH, Method::DELETE])
        // Дозволити будь-які заголовки
        .allow_headers(Any);

     Router::new()  




        .layer(cors)
        .with_state(state)
}