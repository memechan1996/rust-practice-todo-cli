//mod types;
mod todo_mgr;
mod commands;

use crate::todo_mgr::TodoMgr;
use crate::commands::Command;
use crate::todo_mgr::todo::Todo;

use inquire::{ Text };
//use todo_mgr;

fn main() {
    let mut mgr = TodoMgr::new();

    loop{
        let command = Command::select("").prompt().unwrap();

        match command{
            Command::Add => {
                let mut new_todo = Todo{
                    id: 0,
                    title: String::new(),
                    done: false,
                    description: String::new(),
                };
                new_todo.title = Text::new("What is new todo's title?").prompt().unwrap();
                new_todo.description = Text::new("What is new todo's description?").prompt().unwrap();
                new_todo.id = mgr.get_last_id() + 1;

                mgr.add(new_todo);

                println!("New todo added.");
            },
            Command::Delete => {
                let ids: Vec<u64> = Text::new("Please enter the IDs of the todo want to delete.")
                    .prompt().unwrap().split_whitespace().
                    filter_map(|s| s.parse::<u64>().ok()) 
                    .collect();
                mgr.delete(&ids);
                println!("Deleted.");
            },
            Command::List => {
                println!("------------------------------------");
                println!("  id  |       title        | state");
                println!("------------------------------------");
                for todo in mgr.get_todo(){
                    if todo.id == 0 {continue;}
                    println!("{:0>6}|{:<20}|{}", 
                        todo.id, todo.title, 
                        if todo.done {"complete"} else {"pending"}
                    );
                }
                println!("------------------------------------");
            },
            Command::Exit => break,
        }
    }

    println!("See you.");
}
