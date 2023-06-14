//! Structs Modeling Tuber Traders Database tables for Rusty's Root Review
//! Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/1/2023

///Module containing all struct models for Tuber Trader DB tables
pub mod tuber_tables {
    use serde::{Deserialize, Serialize};
    use std::clone::Clone;

    ///Model struct for the selling_price_history table in Tuber Trader
    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub struct SellingPriceHistory {
        pub id: i32,
        pub island: i32,
        //Maps to an id number of a Profile
        pub date: String,
        #[sqlx(rename = "priceAM")]
        pub price_am: i32,
        #[sqlx(rename = "pricePM")]
        pub price_pm: i32,
        pub created_at: String,
        //no date type in rust, so bring it in as a String
        pub updated_at: String,
    }

    ///Model struct for the profile table in Tuber Trader
    #[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
    pub struct Profile {
        pub id: i32,
        //#[sqlx(rename = "islandName")]
        pub island_name: String,
        pub picture: String,
        #[sqlx(rename = "turnipsHeld")]
        pub turnips_held: i32,
        #[sqlx(rename = "pricePaid")]
        pub price_paid: i32,
        //this is an SQL reference to another table
        pub owner_id: i32,
        //maps to an id of a user
        //pub created_at: String,
        //no date type in rust, so bring it in as a String
        //pub updated_at: String,
    }

    ///Model struct for the users table in Tuber Trader
    #[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub password: String,
        pub role: String,
        //in DB is an enum type
        //pub created_at: String,
        //no date type in rust, so bring it in as a String
        //pub updated_at: String,
    }

    ///Model struct for the iphistory table in Tuber Trader
    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub struct IPHistory {
        //pub id: i32,
        pub ip: String,
        //#[sqlx(rename = "user_id")]
        //pub user: i32,
        //maps to a user ID
        //pub created_at: String,
        //no date type in rust, so bring it in as a String
        //pub updated_at: String,
    }
    ///Model struct for the transactions table in Tuber Trader
    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub struct Transactions {
        pub id: i32,
        #[sqlx(rename = "numberSold")]
        pub number_sold: i32,
        #[sqlx(rename = "priceSold")]
        pub price_sold: i32,
        pub profits: i32,
        pub seller: i32,
        //maps to an id of a user
        pub host: i32, //maps to an id of an island Profile
        pub created_at: String,
        //no date type in rust, so bring it in as a String
        pub updated_at: String,
    }
}
