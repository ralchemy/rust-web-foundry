use crate::dtos::ErrorEnvelope;
use application::CreateTaskError;
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub(crate) enum ApiError {
    NotFound,
    MethodNotAllowed,
    InvalidRequest,
    UnsupportedMediaType,
    RequestTooLarge,
    TaskTitleInvalid,
    TaskPolicyRejected,
    TaskPolicyBadResponse,
    TaskPolicyUnavailable,
    Internal,
}

impl From<CreateTaskError> for ApiError {
    fn from(error: CreateTaskError) -> Self {
        match error {
            CreateTaskError::PolicyRejected => Self::TaskPolicyRejected,
            CreateTaskError::PolicyUnavailable => Self::TaskPolicyUnavailable,
            CreateTaskError::PolicyBadResponse => Self::TaskPolicyBadResponse,
            CreateTaskError::Persistence => Self::Internal,
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        match rejection.status() {
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Self::UnsupportedMediaType,
            StatusCode::PAYLOAD_TOO_LARGE => Self::RequestTooLarge,
            _ => Self::InvalidRequest,
        }
    }
}

impl ApiError {
    fn response(&self) -> (StatusCode, ErrorEnvelope) {
        let (status, code, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "not_found", "route not found"),
            Self::MethodNotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                "method_not_allowed",
                "method not allowed",
            ),
            Self::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request is invalid",
            ),
            Self::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                "content type must be application/json",
            ),
            Self::RequestTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "request_too_large",
                "request body is too large",
            ),
            Self::TaskTitleInvalid => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_title_invalid",
                "task title is invalid",
            ),
            Self::TaskPolicyRejected => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_policy_rejected",
                "task policy rejected the title",
            ),
            Self::TaskPolicyBadResponse => (
                StatusCode::BAD_GATEWAY,
                "task_policy_bad_response",
                "task policy returned an invalid response",
            ),
            Self::TaskPolicyUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "task_policy_unavailable",
                "task policy is unavailable",
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            ),
        };
        (status, ErrorEnvelope::new(code, message))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = self.response();
        (status, Json(body)).into_response()
    }
}
