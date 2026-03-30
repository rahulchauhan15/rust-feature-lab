use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct S3UploadRequest {
    pub key: String,
    pub data: String,
}

#[derive(Serialize)]
pub struct S3UploadResponse {
    pub url: String,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

pub async fn upload(
    State(state): State<AppState>,
    Json(payload): Json<S3UploadRequest>,
) -> impl IntoResponse {
    tracing::debug!(key = %payload.key, "handling POST /s3/upload");

    let body = payload.data.into_bytes();

    match state.s3.upload(&payload.key, &body).await {
        Ok(url) => (StatusCode::OK, Json(S3UploadResponse { url })).into_response(),
        Err(err) => {
            tracing::error!(?err, key = %payload.key, "s3 upload failed");
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorBody {
                    error: format!("s3 upload error: {err:#}"),
                }),
            )
                .into_response()
        }
    }
}
