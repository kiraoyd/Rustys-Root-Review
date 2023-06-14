use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
//use jsonwebtoken::{DecodingKey, EncodingKey};

//allow access directly to the .env file
//NOTE: added option for database url for the hardcoded postgres creation of tuber
pub struct EnvOptions {
    pub database_url: String,
    pub mock_tuber_db_url: String,
}

//implement the new function for this struct that maks a new one populated with the URL from the.env
impl EnvOptions {
    pub fn new() -> Self {
        EnvOptions {
            database_url: std::env::var("DATABASE_URL").expect("Missing env var for DATABASE_URL"),
            mock_tuber_db_url: std::env::var("MOCK_TUBER_DB_URL").expect("Missing env var for MOCK_TUBER_DB_URL"),
        }
    }
}

//clippy suggested I add this:
impl Default for EnvOptions {
    fn default() -> Self {
        Self::new()
    }
}

//lets make an error type wrapper for convinience
//AppError contains one of the anyhow structs, 'Error'
pub struct AppError(anyhow::Error);

//here we can implement a method for our AppError struct, that tells axum how to convert AppError into a resposne
//that the server can send back
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0), //show the error message in the AppError
        )
            .into_response()
    }
}

//TODO what the heck is actually going on here?
impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
