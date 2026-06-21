pub mod todo;

use crate::todo_mgr::todo::Todo;

pub struct TodoMgr{
    todos: Vec<Todo>,
}

impl TodoMgr{
    pub fn new() -> Self{
        let mut s = TodoMgr{
            todos: Vec::new(),
        };
        s.todos.push(Todo { 
            id: 0, 
            title: String::from("dummy"), 
            done: true, 
            description: String::from("This is dummy."),
        });
        s
    }

    pub fn add(&mut self, _t: Todo){
        self.todos.push(_t);
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
        self.todos.last().unwrap().id
    }
}