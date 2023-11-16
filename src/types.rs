use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use warp::{filters::ws::Message, reject::Rejection};


pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

/**
 * There is a difference between a client and a user in this case.
 * A user can have several clients —
 * think of the same user connecting to the API using a mobile app and a web app
 */
#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    // list of topic user is interested in
    pub topics: Vec<String>,
    // This sender is used to send messages to this connected client via WebSockets.
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// a data transfer type to register a new client for a user
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    pub user_id: usize,
}

// a data transfer type for successful response to a register request 
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
    pub url: String,
}

// a data transfer type for an event e.g sending a message to a topic's users
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}

// a data transfer type for updating topics for a user's client
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TopicsRequest {
    pub topics: Vec<String>,
}
