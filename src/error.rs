use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Api {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub struct Message {
    message: String,
}

impl IntoResponse for Api {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Api::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
            Api::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, Json(Message { message })).into_response()
    }
}
