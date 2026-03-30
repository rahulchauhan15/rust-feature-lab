use axum::{routing::{get, post}, Router};
use crate::{app_state::AppState, handlers::{weather::get_weather, s3::upload}};

/// Assemble the root [`Router`] with every route and shared state attached.
///
/// Keep this function thin — route declarations only, no business logic.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/weather/:city", get(get_weather))
        .route("/s3/upload", post(upload))
        .with_state(state)
}
