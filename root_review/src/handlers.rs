use axum::http::StatusCode;
use axum::{Extension, Json};
use root_review::AppError;
//use anyhow::Error;
//Use some SERDE here probably, to convert and deal with JSON

use sqlx::PgPool;
//use tracing::log::{error, info};
use crate::models::tuber_tables::IPHistory;

pub async fn get_iph(
    Extension(connection): Extension<PgPool>
) -> Result<(StatusCode, Json<Vec<IPHistory>>), AppError> {
    let res = try_get_iph(&connection).await?;
    Ok(res)
}

//Test route, to see if we can access the DB correctly and make a hit
async fn try_get_iph(
    connection: &PgPool
    //below, the result will carry the status code, and the Json of a Vec of IPHistory struct types
    //or it will carry an anyhow::Error
) -> Result<(StatusCode, Json<Vec<IPHistory>>), anyhow::Error> {
    //change to fetch_all at some point, store in a Vec<SellingPriceHistory>
    let iph: Vec<IPHistory> = sqlx::query_as!(IPHistory, "SELECT ip FROM ip_history")
        .fetch_all(connection)
        .await?;

    Ok((StatusCode::OK, Json(iph)))
}
