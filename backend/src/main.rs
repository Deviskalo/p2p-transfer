use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, channel::mpsc};
use serde_json::Value;
use tokio::sync::{Mutex, broadcast};
use tokio::time::interval;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    connections: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}

/// Starts the server and listens for incoming connections.
///
/// The server is started by spawning a new instance of `axum::Server` and binding it to
/// `0.0.0.0:5000`. The server is then served, and the app is run with the `AppState` shared
/// between all requests.
///
/// The `AppState` contains a `HashMap` of all the active connections, mapped by their
/// connection ID.
#[tokio::main]
async fn main() {
    let state = AppState {
        connections: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/ws", axum::routing::get(websocket_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("Server is running on http://0.0.0.0:5000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Handles a new websocket connection.
///
/// When a new connection is established, the websocket is passed to `handle_socket`
/// for processing.
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handles a new websocket connection.
///
/// This function is called when a new websocket connection is established.
/// It creates a new connection ID, adds the connection to the shared state,
/// and sets up a task to send pings to the client every 30 seconds.
///
/// The function also sets up tasks to forward messages from the client to
/// other connections, and to forward messages from other connections to the
/// client.
///
/// The function returns when the connection is closed.
async fn handle_socket(socket: WebSocket, state: AppState) {
    let conn_id = Uuid::new_v4().to_string();
    let conn_id_clone = conn_id.clone(); // Clone for later use in different tasks

    let (tx, mut rx) = broadcast::channel(100);
    {
        let mut connections = state.connections.lock().await;
        connections.insert(conn_id.clone(), tx.clone());
    }

    let (mut sender, mut receiver) = socket.split();
    let (message_tx, mut message_rx) = mpsc::channel::<Message>(100);

    let sender_task = tokio::spawn(async move {
        while let Some(msg) = message_rx.next().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let ping_tx = message_tx.clone();
    let ping_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(30));
        let mut ping_tx = ping_tx;
        loop {
            interval.tick().await;
            if ping_tx.send(Message::Ping(vec![])).await.is_err() {
                break;
            }
        }
    });

    // Forward messages to other connections
    let forward_tx = tokio::spawn({
        let state = state.clone(); // Clone the state to use it within the async block
        let conn_id = conn_id.clone();
        async move {
            while let Ok(msg) = rx.recv().await {
                let connections = state.connections.lock().await;
                for (id, sender) in connections.iter() {
                    // Skip sending to the current connection
                    if id != &conn_id {
                        let _ = sender.send(msg.clone());
                    }
                }
            }
        }
    });

    let receiver_task = tokio::spawn({
        let state = state.clone();
        let tx = tx.clone();
        let mut target_map: HashMap<String, String> = HashMap::new();
        async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            // Handle registration
                            if data["type"] == "register" {
                                if let Some(id) = data["connectionId"].as_str() {
                                    state
                                        .connections
                                        .lock()
                                        .await
                                        .insert(id.to_string(), tx.clone());
                                }
                                continue;
                            }

                            // Handle message forwarding to target_id
                            if let Some(target_id) = data["target_id"].as_str() {
                                target_map.insert(conn_id.clone(), target_id.to_string());
                                if let Some(target_tx) =
                                    state.connections.lock().await.get(target_id)
                                {
                                    let _ = target_tx.send(Message::Text(text));
                                }
                            }
                        }
                    }
                    Message::Binary(bin_data) => {
                        if let Some(target_id) = target_map.get(&conn_id) {
                            if let Some(target_tx) = state.connections.lock().await.get(target_id) {
                                let _ = target_tx.send(Message::Binary(bin_data));
                            }
                        } else {
                            println!("NotTarget set for binary transfer {}", conn_id);
                        }
                    }
                    Message::Close(_) => break,
                    _ => continue,
                }
            }
        }
    });

    tokio::select! {
        _ = sender_task => {}
        _ = ping_task => {}
        _ = forward_tx => {}
        _ = receiver_task => {}
    }

    // Clean up connections when closed
    state.connections.lock().await.remove(&conn_id_clone);
    println!("Connection {} closed", conn_id_clone);
}
