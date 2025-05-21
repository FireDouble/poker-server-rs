use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::Deserialize;
use crate::engine::Engine;

pub async fn join_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(JoinRequest { name , table}): Json<JoinRequest>,
) -> Result<String, StatusCode> {
    let key = name.clone() + &Utc::now().to_string();
    
    if let Some(table) = engine.lock().unwrap().get_tables().get_mut(table) {
        let result = table.add_player(name, key.clone());
        if !result { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    }
    else {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(key)
}

#[derive(Deserialize)]
pub struct JoinRequest {
    name: String,
    table: usize,
}