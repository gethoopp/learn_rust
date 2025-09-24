
use warp::ws::Message;
use warp::Reply;
use warp::ws::WebSocket;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use std::collections::HashMap;
use futures::{StreamExt, SinkExt};


pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, warp::Rejection>;

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub user_id: usize,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub url: String,
}

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message,warp::Error>>>
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl warp::Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();
    register_client(uuid.clone(), user_id, clients.clone()).await;
    Ok(warp::reply::json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}



pub async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.lock().await.insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}

pub async fn unregister_clinet(id: String, clients: Clients) -> Result<impl Reply> {
    let mut map = clients.lock().await;
    if let Some(client) = map.remove(&id) {
    if let Some(sender) = client.sender{
    let _ = sender.send(Ok(Message::close()));
    }
    println!("Client disconnected: {}", id);
    Ok(warp::reply::json(&serde_json::json!({
         "message": format!("Client {} berhasil dihapus dan koneksi ditutup", id)
    })))
   } else {
    Ok(warp::reply::json(&serde_json::json!({
         "message": format!("Client {} tidak ditemukan", id)
    })))
   }
}


pub async fn unregister_client_all(clients: Clients) -> Result<impl Reply> {
    let mut map = clients.lock().await;
    //close all client 
    for(id,client) in map.drain() {
        if let Some(sender) = client.sender {
          let _ = sender.send(Ok(Message::close()));
        }

        println!("Client Disconnect: {}", id);
    }

     Ok(warp::reply::json(&serde_json::json!({
        "message": "Semua client berhasil dihapus dan koneksi ditutup"
    })))
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl warp::Reply> {
    Ok(ws.on_upgrade(move |socket| handle_ws(socket, id, clients)))
}

pub async fn handle_ws(ws: WebSocket, id: String, clients: Clients) {
    let (mut tx, mut rx) = ws.split();
    let(chat_tx, mut chat_rx) = mpsc::unbounded_channel();

    {
        let mut clients_loc = clients.lock().await;
        if let Some(client) = clients_loc.get_mut(&id) {
            client.sender = Some(chat_tx.clone());
        }
    }

    tokio::spawn(async move {
        while let Some(msg) = chat_rx.recv().await {
            if let Ok(message) = msg {
                let _ = tx.send(message).await;
            }
        }
    });
    println!("Client connected: {}", id);
    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if msg.is_text() {
                println!("Message: {}",msg.to_str().unwrap());
                let _ = chat_tx.send(Ok(Message::text(msg.to_str().unwrap())));
            }
        }
    }
  
}