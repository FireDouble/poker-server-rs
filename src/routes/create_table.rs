use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use chrono::Utc;
use serde::Deserialize;
use crate::engine::Engine;

pub async fn create_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(JoinRequest { name }): Json<JoinRequest>,
) -> String {
    let key = name.clone() + &Utc::now().to_string();

    engine.lock().unwrap().new_table(name, key.clone());

    key
}

#[derive(Deserialize)]
pub struct JoinRequest {
    name: String,
}