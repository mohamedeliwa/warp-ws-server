## route definitions:

- `GET /health`: Indicates if the service is up
- `POST /register`: Registers clients in the application
- `DELETE /register`/{client_id}: Unregisters the client with an ID
- `POST /publish`: Broadcasts an event to clients
- `GET /ws`: The WebSocket endpoint
