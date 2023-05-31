//bring in the handlers crate
use crate::handlers::get_iph;
use crate::handlers::get_big_spender;
//use axum::routing::post;
use axum::routing::get;
use axum::Router;

//make it easy to set routes to the router here, while establishing the router back in main.rs
//Returns a closure that creates a new router with our handlers attached
pub fn routes() -> Router {
    Router::new().route("/", get(get_iph))
    .route("/spender", get(get_big_spender))
}
