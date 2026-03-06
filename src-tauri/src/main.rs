#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_presence::Client;
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct PresencePayload {
    details: String,
    state: String,
}

struct DiscordState {
    client: Arc<Mutex<Client>>,
}

fn main() {
    let mut drpc = Client::new(1479602724554407996); 
    drpc.start();
    let shared_client = Arc::new(Mutex::new(drpc));

    let app_client = shared_client.clone();
    tokio::spawn(async move {
        let app = Router::new().route("/", post(move |Json(payload): Json<PresencePayload>| {
            let client = app_client.clone();
            async move {
                let mut drpc = client.lock().await;
                let _ = drpc.set_activity(|a| a.details(payload.details).state(payload.state));
            }
        }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3020").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    tauri::Builder::default()
        .manage(DiscordState { client: shared_client })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}