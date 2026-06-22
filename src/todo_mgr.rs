//todo_mgr.rs

pub mod todo;

use crate::todo_mgr::todo::Todo;

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoMgr{
    todos: Vec<Todo>,
    next_id: u64,
}

impl TodoMgr{
    pub fn new() -> Self{
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, _t: Todo){
        self.todos.push(_t);
        self.next_id += 1;
    }

    pub fn delete(&mut self, _id:&[u64]){
        let set: std::collections::HashSet<u64> = 
            _id.iter().copied().collect();
        
        self.todos.retain(|todo| !set.contains(&todo.id));
    }

    pub fn get_todo(&self) -> &[Todo]{
        &self.todos
    }

    pub fn get_last_id(&self) -> u64{
        self.next_id
    }

    pub fn save(&self){
        let json = 
            serde_json::to_string_pretty(self).unwrap();
        fs::write("./data/todos.json", json).unwrap();
    }

    pub fn load() -> Self{
        //let json = fs::read_to_string("./data/todos.json");

        match fs::read_to_string("./data/todos.json"){
            Ok(json) => {serde_json::from_str(&json).unwrap()},
            Err(_) => {Self::new()},
        }
    }
}