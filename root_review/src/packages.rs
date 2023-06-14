//! Structs to package data for Rusty's Root Review
//! Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/1/2023

pub mod reply_data {
    use serde::{Deserialize, Serialize};

    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub struct SpenderReply {
        pub island: String,
        pub turnip_quantity: i32,
        pub price_paid: i32,
        pub total_spent: i64,
        pub owner_name: String,
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub struct MaxProfitsReply {
        pub island: String,
        pub turnip_quantity: i32,
        pub price_paid: i32,
        pub total_spent: i64,
        pub owner_name: String,
        pub potential_profits: i64,
        pub selling_price: i32,
        pub profited: bool,
    }
}
