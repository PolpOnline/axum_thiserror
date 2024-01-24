pub use axum_thiserror_macro::ErrorStatus;

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use thiserror::Error;

    #[derive(Error, Debug, ErrorStatus)]
    pub enum Test {
        #[error("Hello !")]
        #[status(StatusCode::ACCEPTED)]
        Hello,
    }

    #[test]
    pub fn assert_compiles() {
        let t = Test::Hello;
        let response: Response = t.into_response();
        assert_eq!(response.status(), StatusCode::ACCEPTED)
    }
}
