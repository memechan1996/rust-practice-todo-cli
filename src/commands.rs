//commands.rs

use inquire_derive::Selectable;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Selectable)]
pub enum Command{
    Add,
    Delete,
    List,
    Exit,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Command::Add => write!(f, "Add"),
            Command::List => write!(f, "List"),
            Command::Delete => write!(f, "Delete"),
            Command::Exit => write!(f, "Exit"),
        }
    }
}