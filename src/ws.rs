use crate::types::{Client, Clients, TopicsRequest};
use futures::FutureExt;
use futures::StreamExt;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::Message;
use warp::filters::ws::WebSocket;

// establishes a web socket connection for a user client
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    // any msg received from the client should be forwared to the sw sink
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    {
        client.sender = Some(client_sender);
        clients.lock().unwrap().insert(id.clone(), client);
        println!("{} connected", id);
    }
    // processing any msg received from the ws stream
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    {
        clients.lock().unwrap().remove(&id);
        println!("{} disconnected", id);
    }
}

// process messages and events received from the web socket
async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    // replying to a ping-pong health checking message 
    if message == "ping" || message == "ping\n" {
        println!("Pong! is should be sent!");
        let locked = clients.lock().unwrap();
        println!("Pong! got the lock!");
        match locked.get(id) {
            Some(v) => {
                println!("Pong! found the client!");
                if let Some(sender) = &v.sender {
                    println!("Pong! is sent!");
                        let _ = sender.send(Ok(Message::text("Pong!")));
                }
            }
            None => return,
        };

        return;
    }

    // updating topic the client is interested in
    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error while parsing message to topics request: {}", e);
            return;
        }
    };

    let mut locked = clients.lock().unwrap();
    match locked.get_mut(id) {
        Some(v) => {
            v.topics = topics_req.topics;
        }
        None => return,
    };
}
