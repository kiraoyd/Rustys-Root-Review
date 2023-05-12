//use axum::http::StatusCode;
//use axum::response::{IntoResponse, Response};
//use jsonwebtoken::{DecodingKey, EncodingKey};
//use serde::{Deserialize, Serialize};

//allow access directly to the .env file
pub struct EnvOptions {
    pub database_url: String,
}

//implement the new function for this struct that maks a new one populated with the URL from the.env
impl EnvOptions {
    pub fn new() -> Self {
        EnvOptions {
            database_url: std::env::var("DATABASE_URL").expect("Missing env var for DATABASE_URL"),
        }
    }
}
