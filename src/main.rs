use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::Filter;

mod handler;
mod types;
mod ws;

use types::Clients;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // checks the health of the server
    let health_route = warp::path("health").map(handler::health_handler);

    let register = warp::path("register");
    // registers a new user
    let register_route = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler);
    // unregisters a user
    let unregister_route = register
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::unregister_handler);

    // publishes a message to users who follows a specific topic
    let publish = warp::path("publish")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    // connects users to web socket connection
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_route)
        .or(unregister_route)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// creates and returns a warp filter,
// with clients reference as its extracted value
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
