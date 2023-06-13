//! Rusty's Root Review: a selection of analytic GET routes for Tuber Trader's Island Profiles
//! Author: Kira Klingenberg (with some code pulled directly from Doggr_w23's auth_rs microservice, by Casey Bailey)
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/1/2023


///The majority of the code here in main.rs provided courtesy of Casey Bailey from this repo: https://github.com/kiraoyd/doggr_w23/tree/master/auth_rs
///Notes on my understanding of how this code works can be found in JOURNAL.md, and in the doc comments

use anyhow::Result as AnyResult;
use dotenvy::dotenv;
use root_review::EnvOptions;

//alias for PoolOptions, specialized for postgres
use sqlx::postgres::PgPoolOptions;

//alias for Pool, specialized for postgres
use sqlx::PgPool;

//use root_review::EnvOptions;
use axum::Extension;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::log::info;
use tracing::trace;
//use tracing_subscriber;

//all extra files that are not main.rs and lib.rs need to be published at the top of the crate module tree, here or in lib.rs
pub mod handlers;
pub mod models;
pub mod routes;
pub mod packages;
pub mod db;

use routes::routes;
use db::create_mock_db;


//Set up the axum server to run
///Sets up the axum server to connect to the DB, layers on cors middlewares, our Router, and the DB connection
async fn run() -> AnyResult<()> {
    //here we get back out pool of DB connections
    let connection = establish_connection().await?;
    //Set up cors middlewares
    let cors = CorsLayer::new().allow_origin(Any);
    //Register our routes, stack middle ware and the DB connection onto the router to get access
    let app = routes().layer(cors).layer(Extension(connection));

    //allow the server to listen to anything. Safe thanks to docker.
    //create a new SocketAddr struct with the Ip Address of [0,0,0,0] and the port of 3333
    let addr = SocketAddr::from(([0, 0, 0, 0], 3333));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    //TODO could add an unwrap() here? But would that obselete the Ok(()) below?
    //if we make it through all .awaits to completion, return that we were successful
    Ok(())
}

///Establishes a pool of connections to Tuber Trader's DB
/// DB information stored in the .env file
pub async fn establish_connection() -> AnyResult<PgPool> {
    info!("Establishing Database connection...");

    //connect takes a reference string to the connection URL we hve in the .env
    let env_opts = EnvOptions::new(); //make new struct to access .env contents
                                      //right now all we need in that struct is the database URL, but more could be added

    // create a pool of connections!
    //chaining together some functions to create a pool with max connections set and connected to our tuber DB
    //TODO use these lines when connecting to REAL tuber trader DB
    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&env_opts.database_url)
    //     .await?;
//    info!("Database connection to tuber established!");

    //TODO comment out when not testing against mock tuber
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env_opts.mock_tuber_db_url)
        .await?;
    //convert pool from Result to Option, and extract result value
    info!("Database connection to mock tuber established!");
    //If connection is successful, we can return the pool
    Ok(pool)
}

///Runs the server, start it listening
/// Sets up a tokio runtime
#[tokio::main]
async fn main() -> AnyResult<()> {
    //set up dotenv
    dotenv().ok();
    //initialize logging
    tracing_subscriber::fmt::init();
    trace!("App initialized.");

    //create the mock DB and seed it
    create_mock_db().await.unwrap();
    //Set our serve to listen
    //cd ..run().await.unwrap();
    Ok(())
}
