//todo.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo{
    pub id: u64,
    pub title: String,
    pub done: bool,
    pub description: String,
}