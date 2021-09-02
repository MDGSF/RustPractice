use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::Extension;
use axum::handler::get;
use axum::response::{Html, IntoResponse};
use axum::AddExtensionLayer;
use axum::Router;
use futures::{sink::SinkExt, stream::StreamExt};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

struct AppState {
  user_set: Mutex<HashSet<String>>,
  tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
  let user_set = Mutex::new(HashSet::new());
  let (tx, _rx) = broadcast::channel(100);
  let app_state = Arc::new(AppState { user_set, tx });

  let app = Router::new()
    .route("/", get(index))
    .route("/websocket", get(websocket_handler))
    .layer(AddExtensionLayer::new(app_state));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> Html<&'static str> {
  Html(std::include_str!("../chat.html"))
}

async fn websocket_handler(
  ws: WebSocketUpgrade,
  Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
  ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
  // By splitting, we can send and receive at the same time.
  let (mut sender, mut receiver) = stream.split();

  let mut username = String::new();

  while let Some(Ok(message)) = receiver.next().await {
    if let Message::Text(name) = message {
      check_username(&state, &mut username, &name);

      if !username.is_empty() {
        break;
      } else {
        let _ = sender
          .send(Message::Text(String::from("Username already taken.")))
          .await;
        return;
      }
    }
  }

  let mut rx = state.tx.subscribe();

  // Send joined message to all subscribers.
  let msg = format!("{} joined", username);
  let _ = state.tx.send(msg);

  // This task will receive broadcast messages and
  // send text message to our client.
  let mut send_task = tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
      if sender.send(Message::Text(msg)).await.is_err() {
        break;
      }
    }
  });

  let tx = state.tx.clone();
  let name = username.clone();

  // This task will receive messages from client and
  // send them to broadcast subscribers.
  let mut recv_task = tokio::spawn(async move {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
      let _ = tx.send(format!("{}: {}", name, text));
    }
  });

  tokio::select! {
    _ = (&mut send_task) => recv_task.abort(),
    _ = (&mut recv_task) => send_task.abort(),
  };

  let msg = format!("{} left", username);
  let _ = state.tx.send(msg);

  state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
  let mut user_set = state.user_set.lock().unwrap();

  if !user_set.contains(name) {
    user_set.insert(name.to_owned());
    string.push_str(name);
  }
}
