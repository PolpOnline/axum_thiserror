use axum::{http::StatusCode, response::IntoResponse};
use axum_thiserror::ErrorStatus;
use thiserror::Error;

#[derive(Error, Debug, ErrorStatus)]
pub enum UserCreateError {
    #[error("Invalid email {email}")]
    #[status(StatusCode::UNAUTHORIZED)]
    InvalidEmail { email: String },
    #[error("User {0} already exists with email")]
    #[status(StatusCode::CONFLICT)]
    UserAlreadyExists(String, String),
}

#[test]
fn basic_error() {
    let error = UserCreateError::InvalidEmail {
        email: "user01".to_string(),
    };
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let error = UserCreateError::UserAlreadyExists("user01".to_string(), "email".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}
