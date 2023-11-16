# Warp-WS

- A simple web-socket server built using Rust and warp which is an easy, composable, web server framework.
- It's built for practicing Rust and async Rust.
- It allows simple functionality like
  - server health checking route.
  - connecting a user to through a web socket,
  - publishing messages to users who are interested in a certain topic
  - allowing the user to update the topics he/she is interested in.

## route definitions:

- `GET /health`: Indicates if the service is up
- `POST /register`: Registers clients in the application
- `DELETE /register`/{client_id}: Unregisters the client with an ID
- `POST /publish`: Broadcasts an event to clients
- `GET /ws`: The WebSocket endpoint
  - send 'ping' -> server responds by a 'pong', just for health checking the connection.
  - send a json with this structure `{"topics" : ["topic", "another-topic"]}` to update the topics a client is interested in
  - any other message sent through the web-socket will not be handled, you're free to add more message types e.g Event type to receive and send message through the web-socket.
