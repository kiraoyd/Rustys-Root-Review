use axum::http::StatusCode;
use axum::{Extension, Json};
use root_review::AppError;
//use tracing::log::Record; //needed to use query! to catch the results

use sqlx::PgPool;
//use tracing::log::{error, info};
use crate::models::tuber_tables::IPHistory;
use crate::models::tuber_tables::Profile;


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
    let iph: Vec<IPHistory> = sqlx::query_as!(IPHistory, "SELECT ip FROM iphistory")
        .fetch_all(connection)
        .await?;

    Ok((StatusCode::OK, Json(iph)))
}


pub async fn get_big_spender(
    Extension(connection): Extension<PgPool>
) -> Result<(StatusCode, Json<Profile>), AppError>{
    let res = try_biggest_spender(&connection).await?;
    Ok(res)
}
//Get and collect the turnips_held and price_paid for a all islands in DB
//calculate the total bells spent on turnips for each island
//and determine the biggest spender. Return the island that gets this honor.
//The SQL query text has to match the Rust structs representing them
async fn try_biggest_spender(
    connection: &PgPool
) -> Result<(StatusCode, Json<Profile>), anyhow::Error>{

    //get all islands from Profile table, collect into a Vector
    //TODO query! returns a Record type....
    let all_islands: Vec<Profile> = sqlx::query_as!(Profile, "SELECT id, island_name, picture, turnips_held, price_paid FROM profile")
        .fetch_all(connection)
        .await?;

    //let total_islands = all_islands.len();

    //from this vector of profile stucts, calculate each islands bells spent on turnips, and save the max
    let mut biggest_spender: Profile = all_islands[0].clone(); //assume the first one is the biggest
   // let mut biggest = 0; //will hold the spending of the current biggest spender
    //let mut spent = 0;
    //I want to save the profile of the biggest spender
    for island in all_islands {
        let biggest = biggest_spender.price_paid * biggest_spender.turnips_held;
        let spent = island.price_paid * island.turnips_held;
        if spent > biggest {
            biggest_spender = island.clone();
        }
    }
    Ok((StatusCode::OK, Json(biggest_spender)))
}