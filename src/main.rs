use axum::{
    routing::post,
    Router,
    Json,
    Extension,
};
use std::net::SocketAddr;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type SharedState = Arc<RwLock<State>>;

#[derive(Default)]
struct State {
    db: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/key/get", post(kv_get))
        .route("/key/set", post(kv_set))
        .layer(Extension(SharedState::default()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3010));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn kv_get(
    Json(payload): Json<serde_json::Value>,
    Extension(state): Extension<SharedState>,
) -> Json<Value> {
    let db = &state.read().unwrap().db;
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    if let Some(value) = db.get(&key) {
        Json(json!({ "status": 0, "data": value }))
    } else {
        Json(json!({ "status": 1, "data": "" }))
    }
}

async fn kv_set(
    Json(payload): Json<serde_json::Value>,
    Extension(state): Extension<SharedState>,
) {
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    let value = payload.as_object().unwrap().get("value").unwrap().as_str().unwrap().to_string();
    state.write().unwrap().db.insert(key, value);
}