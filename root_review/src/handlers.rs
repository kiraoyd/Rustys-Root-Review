
use axum::http::StatusCode;
use axum::{Extension, Json};
use root_review::{AppError};
//use anyhow::Error;

use sqlx::PgPool;
//use tracing::log::{error, info};

//TODO how to bring in the models from models.rs?
use crate::models::{models};
//Use some SERDE here probably, to convert and deal with JSON

//TODO write an actual route!
pub async fn get_users(
    Extension(connection): Extension<PgPool>, Json(credentials): Json<User>,
) ->  Result<StatusCode, AppError> {
    let res = try_get_user(&connection, &credentials).await?;
    Ok(res)
}

async fn try_get_user(connection: &PgPool, credentials: &User) -> Result<StatusCode, anyhow::Error> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT id, name FROM user").fetch_all(connection).await?;
    Ok(users)
}


