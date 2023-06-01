//! Router for Rusty's Root Review
//! Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/1/2023

//bring in the handlers
use crate::handlers::get_iph;
use crate::handlers::get_big_spender;
use crate::handlers::get_max_profits;
use crate::handlers::get_most_profitable_island;
use axum::routing::get;
use axum::Router;

///Router for Rust's Root Review axum server
//make it easy to set routes to the router here, while establishing the router back in main.rs
//Returns a closure that creates a new router with our handlers attached
pub fn routes() -> Router {
     Router::new().route("/", get(get_iph))  //POSTMAN: localhost:3333/
    .route("/spender", get(get_big_spender)) //POSTMAN: localhost:3333/spender
    .route("/profits/:selling_price/:island_id", get(get_max_profits)) //POSTMAN: localhost:3333/profits/262/6
     .route("/profits/:selling_price", get(get_most_profitable_island)) //POSTMAN: localhost:3333/profits/110
}
