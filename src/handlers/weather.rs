use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::app_state::AppState;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}
pub async fn get_weather(
    State(state): State<AppState>,
    Path(city):   Path<String>,
) -> impl IntoResponse {
    tracing::debug!(city, "handling GET /weather/{city}");

    match state.weather.get_weather(&city).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),

        Err(err) => {
            tracing::error!(?err, city, "weather service returned an error");
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorBody {
                    error: format!("upstream error: {err:#}"),
                }),
            )
                .into_response()
        }
    }
}
