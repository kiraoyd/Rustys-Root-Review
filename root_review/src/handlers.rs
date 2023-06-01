use axum::http::StatusCode;
use axum::{Extension, Json};
use root_review::AppError;

//use tracing::log::Record; //needed to use query! to catch the results

use sqlx::PgPool;
//use tracing::log::{error, info};
use crate::models::tuber_tables::IPHistory;
use crate::models::tuber_tables::Profile;
use crate::models::tuber_tables::User;
use crate::models::tuber_tables::SpenderReply;


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

//localhost:3333/spender
pub async fn get_big_spender(
    Extension(connection): Extension<PgPool>
) -> Result<(StatusCode, Json<SpenderReply>), AppError>{
    let res = try_biggest_spender(&connection).await?;
    Ok(res)
}
//Get and collect the turnips_held and price_paid for a all islands in DB
//calculate the total bells spent on turnips for each island
//and determine the biggest spender. Return the island that gets this honor.
//The SQL query text has to match the Rust structs representing them
async fn try_biggest_spender(
    connection: &PgPool
) -> Result<(StatusCode, Json<SpenderReply>), anyhow::Error>{

    //get all islands from Profile table, collect into a Vector
    let all_islands: Vec<Profile> = sqlx::query_as!(Profile, "SELECT id, island_name, picture, turnips_held, price_paid, owner_id FROM profile")
        .fetch_all(connection)
        .await?;

    //let total_islands = all_islands.len();

    //from this vector of profile stucts, calculate each islands bells spent on turnips, and save the max
    let mut biggest_spender: Profile = all_islands[0].clone(); //assume the first one is the biggest
    let mut spent: i64 = 0;
    //I want to save the profile of the biggest spender
    for island in all_islands {
        let biggest: i64 = biggest_spender.price_paid as i64 * biggest_spender.turnips_held as i64;
        spent = island.price_paid as i64 * island.turnips_held as i64;
        if spent > biggest {
            biggest_spender = island.clone();
        }
    }

    let owner_id: i32 = biggest_spender.owner_id;

    //use the owner_id number in the biggest_spender, to query the User table and find out the name of the user that owns the biggest spending island
    let owner = sqlx::query!("SELECT name FROM users WHERE id = $1", owner_id).fetch_one(connection).await?;
    //make a new Spender_reply struct and populate with info from the biggest spender and the calculated spending amount
    let reply = SpenderReply {
        island: biggest_spender.island_name.clone(),
        turnip_quantity: biggest_spender.turnips_held.clone(),
        price_paid: biggest_spender.price_paid.clone(),
        total_spent: spent,
        owner_name: owner.name,
    };

    //package up the relevant info from the Profile, into a special struct to send back
    Ok((StatusCode::OK, Json(reply)))
}