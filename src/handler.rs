use crate::types::{Client, Clients, Event, RegisterRequest, RegisterResponse, Result};
use uuid::Uuid;
use warp::{
    filters::ws::Message,
    http::StatusCode,
    reply::{json, with_status, Reply},
};

// Returns an empty Reply with status code 200 OK
pub fn health_handler() -> impl Reply {
    StatusCode::OK
}

// registering a client for a user
pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), user_id, clients).await;

    Ok(with_status(
        json(&RegisterResponse {
            url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
        }),
        StatusCode::CREATED,
    ))
}

// inserts a user to the memory database
async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.lock().unwrap().insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}

// unregistering a client for a user
pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    // removing the user from the database
    clients.lock().unwrap().remove(&id);
    Ok(StatusCode::OK)
}

// connecting via websockets
pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.lock().unwrap().get(&id).cloned();
    match client {
        Some(c) => {
            Ok(ws.on_upgrade(move |socket| crate::ws::client_connection(socket, id, clients, c)))
        }
        None => Err(warp::reject::not_found()),
    }
}

//  broadcast messages to connected clients.
pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    println!("{:?}", body);
    clients
        .lock()
        .unwrap()
        .iter_mut()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id != v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}
