//bring in the handlers crate
use crate::handlers::test_route;

use axum::routing::post;
use axum::Router;

//make it easy to set routes to the router here, while establishing the router back in main.rs
//Returns a closure that creates a new router with our handlers attached
pub fn routes() -> Router {
    Router::new().route("/", post(test_route))
    //.route("", http(other_route))
}
