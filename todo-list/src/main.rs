/*
    📓 :
        - Todo Taking App
        - Work on terminal
        - Should record info like
            label
            completed
            id
            createAt
            dueAt
        - should have function like
            - list todo
            - add new todo item
            - update todo item
            - delete todo

*/

// implement the trait todoAction.

use serde::Serialize;
use std::{
    fs::File,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

trait TodoAction {
    fn add(&mut self, description: &str) -> bool;
    fn update(&mut self, id: &str) -> Result<(), String>;
    fn remove(&mut self, id: &str) -> bool;
    fn save_to_file(&mut self) -> ();
}

#[derive(Debug, Serialize)]
struct TodoItem {
    description: String,
    completed: bool,
    id: u64,
}

#[derive(Debug, Serialize)]
struct Todo {
    list: Vec<TodoItem>,
}

// prepare constrcutor for Todo.

impl Todo {
    // function to create new todo
    fn new() -> Self {
        Todo { list: Vec::new() }
    }

    // function to list todo data.
    fn list(&self) {
        println!("{:=^40}", " 📚 todo to focus 📚 ");
        println!("");

        for todo in &self.list {
            let status = if todo.completed { '✅' } else { '❎' };
            println!("{status} {}", todo.description);
            println!("");
        }

        println!("");
        println!("{:=^40}", " 💚 thank you 💚 ");
    }
}

impl TodoAction for Todo {
    fn add(&mut self, description: &str) -> bool {
        let _todo = TodoItem {
            description: description.to_string(),
            completed: false,
            id: self.list.len() as u64 + 1,
        };

        self.list.push(_todo);

        true
    }

    fn update(&mut self, id: &str) -> Result<(), String> {
        let _id: u64 = id.parse().expect("Please enter number valid number");

        // use iter_mut to get the mutable ref and find to locate the item.

        if let Some(_todo) = self.list.iter_mut().find(|x| x.id == _id) {
            _todo.completed = !_todo.completed;
            Ok(())
        } else {
            Err("Task not found!".to_string())
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let _id: u64 = id.parse().expect("Please enter number valid number");

        let initial_len = self.list.len();

        self.list.retain(|x| x.id != _id);

        self.list.len() != initial_len
    }

    fn save_to_file(&mut self) {
        println!("saving todo list to file!");
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let data = serde_json::to_string_pretty(self).expect("error while parsing!");
        let file_name = format!("todo-{time}.json");
        let mut file = File::create(&file_name).expect("failed create file!");
        file.write_all(data.as_bytes())
            .expect("failed to write to file");
        println!("Cheers 🎉, Todo saved to file {file_name}!");
    }
}

fn display_help() {
    println!("➡");
    println!("");
    println!("Please choose an action!");
    println!();
    println!("Add New Todo      : Press `i`");
    println!("Update Todo       : Press `u`");
    println!("Remove Todo       : Press `d`");
    println!("List Todos        : Press `p`");
    println!("Help Todo         : Press `h`");
    println!("Quit Todo         : Press `q`");
    println!();
    println!("---------------------------------------");
    println!("🎉 Welcome to Your Todo App! 🎉");
    println!("Use the commands above to manage your tasks effortlessly.");
    println!("Have fun organizing your life!");
    println!();
}

enum TodoCommand {
    Add,
    Update,
    Remove,
    List,
    Help,
    Quit,
    Save,
}

impl TodoCommand {
    fn from_input(input: &str) -> Option<TodoCommand> {
        match input {
            "i" => Some(TodoCommand::Add),
            "u" => Some(TodoCommand::Update),
            "d" => Some(TodoCommand::Remove),
            "p" => Some(TodoCommand::List),
            "h" => Some(TodoCommand::Help),
            "q" => Some(TodoCommand::Quit),
            "s" => Some(TodoCommand::Save),
            _ => None,
        }
    }
}

// to read the input from cammand line
fn read_input() -> String {
    let mut user_inupt = String::new();
    io::stdin()
        .read_line(&mut user_inupt)
        .expect("Failed to read input");

    let input = user_inupt.trim();
    input.to_string()
}

pub fn todo_program() {
    let mut notes = Todo::new();
    loop {
        match TodoCommand::from_input(&read_input()) {
            Some(TodoCommand::Add) => {
                println!("What you wanna do ?");
                notes.add(&read_input());
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::Update) => {
                println!("Enter id of task to update!");
                match notes.update(&read_input()) {
                    Ok(_) => println!("Task updated successfully!"),
                    Err(e) => println!("error occured {}", e),
                }
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::Remove) => {
                println!("Enter id of task to remove!");
                if notes.remove(&read_input()) {
                    println!("Task removed successfully!");
                } else {
                    println!("Failed to remove");
                }
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::List) => notes.list(),
            Some(TodoCommand::Help) => {
                display_help();
            }
            Some(TodoCommand::Quit) => {
                break;
            }
            Some(TodoCommand::Save) => notes.save_to_file(),
            None => {
                println!("Invalid command!");
            }
        }
        println!("");
    }
}

fn main() {
    println!("🎉 Welcome to Your Note 📚 taking App! 🎉");
    println!("--- Press `h` for help or to learn more");

    todo_program();
}
