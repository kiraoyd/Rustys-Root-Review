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

//bring in routes
mod handlers;
mod routes;
use routes::routes;

//Set up the axum server to run

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

//Just creates DB connection
pub async fn establish_connection() -> AnyResult<PgPool> {
    info!("Establishing Database connection...");

    //connect takes a reference string to the connection URL we hve in the .env
    let env_opts = EnvOptions::new(); //make new struct to access .env contents
                                      //right now all we need in that struct is the database URL, but more could be added

    // create a pool of connections!
    //chaining together some functions to create a pool with max connections set and connected to our tuber DB
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env_opts.database_url)
        .await?;

    //convert pool from Result to Option, and extract result value
    info!("Database connection established!");
    //If connection is successful, we can return the pool
    Ok(pool)
}

//Run the server, start it listening
#[tokio::main]
async fn main() -> AnyResult<()> {
    //set up dotenv
    dotenv().ok();
    //initialize logging
    tracing_subscriber::fmt::init();
    trace!("App initialized.");

    //Set our serve to listen
    run().await.unwrap();
    Ok(())
}
