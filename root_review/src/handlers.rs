//! Handlers for Rusty's Root Review routes
//! Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/1/2023

use axum::http::StatusCode;
use axum::{Extension, Json};
use root_review::AppError;
use axum::extract::Path;
//use uuid::Uuid;
use std::str::FromStr;

//use tracing::log::Record; //needed to use query! to catch the results
use sqlx::PgPool;
//use tracing::log::{error, info};
use crate::models::tuber_tables::IPHistory;
use crate::models::tuber_tables::Profile;
//use crate::models::tuber_tables::User;

use crate::packages::reply_data::SpenderReply;
use crate::packages::reply_data::MaxProfitsReply;

///public GET route to retrieve all IP numbers from the IPHistory table
///No use beyond testing a connection to the DB and res/reply success
pub async fn get_iph(
    Extension(connection): Extension<PgPool>
) -> Result<(StatusCode, Json<Vec<IPHistory>>), AppError> {
    let res = try_get_iph(&connection).await?;
    Ok(res)
}

///Implements the get_iph route functionality
/// Retrieves all IP values from the database
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

///public GET route to retrieve all islands info
/// calculates how much each spent on turnips currently, and return the info for the biggest spender
pub async fn get_big_spender(
    Extension(connection): Extension<PgPool>
) -> Result<(StatusCode, Json<SpenderReply>), AppError>{
    let res = try_biggest_spender(&connection).await?;
    Ok(res)
}

///Implements the get_big_spender route functionality
/// Retrieves all island profile info from the Profile table, calculated how much each has spent, saves the infor for the biggest spender
/// Queries the User table to get the name of the User that owns the bigest spending island
/// Packages up island, user and total spent info and sends this back as a JSON in the reply
async fn try_biggest_spender(
    connection: &PgPool
) -> Result<(StatusCode, Json<SpenderReply>), anyhow::Error>{

    //get all islands from Profile table, collect into a Vector
    let all_islands: Vec<Profile> = sqlx::query_as!(Profile, "SELECT id, island_name, picture, turnips_held, price_paid, owner_id FROM profile")
        .fetch_all(connection)
        .await?;

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
        island: biggest_spender.island_name,
        turnip_quantity: biggest_spender.turnips_held,
        price_paid: biggest_spender.price_paid,
        total_spent: spent,
        owner_name: owner.name,
    };

    //package up the relevant info from the Profile, into a special struct to send back
    Ok((StatusCode::OK, Json(reply)))
}


///public GET route to retrieve the potential profits for a specific island, given a specified selling price value
///Island to search for is based on id#
/// Island id and selling price value to work with are provded as params in the route URL
//route that captures from the URL using Path and calculates max_profits possible for the given selling price for an island
pub async fn get_max_profits(
    Extension(connection): Extension<PgPool>, Path((selling_price, island_id)): Path<(String, String)>,
) -> Result<(StatusCode, Json<MaxProfitsReply>), AppError>{
    let res = try_max_profits(&connection, (selling_price, island_id)).await?;
    Ok(res)
}

///Implements the get_max_profits route functionality
/// Extracts the requested island ID and selling price value to work with, from the URL params
/// Queries the Profile table to get the information for the island requested
/// Calculates the total spent, the total earned if sold at the selling price, and the profits (or losses) that result
/// Packages up island information, calculated results and profit/loss status and sends back this info as a JSON in the reply
async fn try_max_profits(
    connection: &PgPool, (selling_price, island_id) : (String, String), //pattern matching!
) -> Result<(StatusCode, Json<MaxProfitsReply>), anyhow::Error>{

    //parse out the Params into i32's
    let num_selling_price = i32::from_str(&selling_price).expect("error parsing");
    let num_island_id = i32::from_str(&island_id).expect("error parsing");

    //Query to find the island in question, then calculate the profits based on the given selling price
    let requested_island = sqlx::query!("SELECT island_name, turnips_held, price_paid, owner_id FROM profile WHERE id = $1", num_island_id)
        .fetch_one(connection).await?;

    //use the owner_id number in the biggest_spender, to query the User table and find out the name of the user that owns the biggest spending island
    let owner_id: i32 = requested_island.owner_id;
    let owner = sqlx::query!("SELECT name FROM users WHERE id = $1", owner_id).fetch_one(connection).await?;

    let spent:i64 = (requested_island.turnips_held * requested_island.price_paid).into();
    let earned: i64 = (requested_island.turnips_held * num_selling_price).into();
    let profits = earned - spent;

    let mut profit_result: bool = true;
    if profits <= 0{
        profit_result = false;
    }

    //make a new MaxProfitsReply struct and populate with info
    let reply = MaxProfitsReply {
        island: requested_island.island_name,
        turnip_quantity: requested_island.turnips_held,
        price_paid: requested_island.price_paid,
        total_spent: spent,
        owner_name: owner.name,
        potential_profits: profits,
        selling_price: num_selling_price,
        profited: profit_result,
    };

    Ok((StatusCode::OK, Json(reply)))
}

///public GET route to retrieve the island that will reap the largest profit given a specified selling price value
///Selling price value to work with is provded as a param in the route URL
//route that captures from the URL using Path and calculates max_profits possible for the given selling price for an island
pub async fn get_most_profitable_island(
    Extension(connection): Extension<PgPool>, Path(selling_price): Path<String>,
) -> Result<(StatusCode, Json<MaxProfitsReply>), AppError>{
    let res = try_most_profitable_island(&connection, selling_price).await?;
    Ok(res)
}


///Implements the get_most_profitable_island route functionality
/// Extracts a specified selling price from the URL params, queries the Profile table to get all island info
/// Uses selling price to calculate possible profits for all islands at that price
/// Tracks the island profile with the maximum profit potential, and the values representing what that island spent and what the profit value is
/// Stores the results in a MaxProfitsReply struct to be returned as a JSON with the reply
async fn try_most_profitable_island(
    connection: &PgPool, selling_price : String, //pattern matching!
) -> Result<(StatusCode, Json<MaxProfitsReply>), anyhow::Error> {

    //parse out the Param into i32
    let num_selling_price = i32::from_str(&selling_price).expect("error parsing");

    //get all islands from Profile table, collect into a Vector
    let all_islands: Vec<Profile> = sqlx::query_as!(Profile, "SELECT id, island_name, picture, turnips_held, price_paid, owner_id FROM profile")
        .fetch_all(connection)
        .await?;

    //iterate through the islands, calculate the profits made on the given selling price, save the island with the highest profit
    let mut most_profitable: Profile = all_islands[0].clone(); //assume the first one makes the most in profit
    let mut spent: i64 = (most_profitable.turnips_held * most_profitable.price_paid).into();
    let mut earned: i64 = (most_profitable.turnips_held * num_selling_price).into();
    let mut profits = earned - spent;

    let mut max_profits: i64 = profits;
    let mut save_spent: i64 = spent;

    for island in all_islands {
        spent = (island.turnips_held * island.price_paid).into();
        earned = (island.turnips_held * num_selling_price).into();
        profits = earned - spent;
        if profits > max_profits {
            max_profits = profits;
            save_spent = spent;
            most_profitable = island.clone();
        }
    }


    //use the owner_id number in the biggest_spender, to query the User table and find out the name of the user that owns the biggest spending island
    let owner_id: i32 = most_profitable.owner_id;
    let owner = sqlx::query!("SELECT name FROM users WHERE id = $1", owner_id).fetch_one(connection).await?;

    //it might be possible that ALL islands were at a loss, so still important to mark this bool
    let mut profitable = true;
    if max_profits <= 0{
        profitable = false;
    }

    //make a new MaxProfitsReply struct and populate with info
    let reply = MaxProfitsReply {
        island: most_profitable.island_name,
        turnip_quantity: most_profitable.turnips_held,
        price_paid: most_profitable.price_paid,
        total_spent: save_spent,
        owner_name: owner.name,
        potential_profits: max_profits,
        selling_price: num_selling_price,
        profited: profitable,
    };

    Ok((StatusCode::OK, Json(reply)))
}