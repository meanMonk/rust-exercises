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

use std::{fs::File, io::{self, Write}, time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;

trait TodoAction {
    fn add(&mut self, description: &str) -> bool;
}

#[derive(Debug,Serialize)]
struct TodoItem {
    description: String,
    completed: bool,
    id: u64,
}

#[derive(Debug,Serialize)]
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
        for todo in &self.list {
            let status = if todo.completed { '✅' } else { '❎' };
            println!("{status} {}", todo.description);
            println!("");
        }
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
    
}

fn display_help() {
    println!("➡");
    println!("");
    println!("Please choose an action!");
    println!();
    println!("📝Add New Todo        : Press `i`");
    println!("✏️Edit Todo           : Press `e`");
    println!("🗑️Remove Todo         : Press `d`");
    println!("📋List Todos          : Press `p`");
    println!("ℹ️Help Todo           : Press `h`");
    println!("🔚Quit Todo           : Press `q`");
    println!();
    println!("---------------------------------------");
    println!("🎉 Welcome to Your Todo App! 🎉");
    println!("Use the commands above to manage your tasks effortlessly.");
    println!("Have fun organizing your life!");
    println!();
}

enum TodoCommand {
    Add,
    Edit,
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
            "e" => Some(TodoCommand::Edit),
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
                println!("Enter new task below");
                notes.add(&read_input());
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::Edit) => {
                println!("TodoCommand::Edit");
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::Remove) => {
                println!("TodoCommand::Remove");
                println!("i: new todo e: edit q: quit s:save");
            }
            Some(TodoCommand::List) => {
                println!("{:=^40}","📚 todo to focus");
                println!("");
                notes.list();
                println!("");
                println!("{:=^40}","💚 thank you 💚 ");
            }
            Some(TodoCommand::Help) => {
                display_help();
            }
            Some(TodoCommand::Quit) => {
                println!("TodoCommand::Quit");
                break;
            }
            Some(TodoCommand::Save) => {
                println!("saving todo list to file!");
                let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let data = serde_json::to_string_pretty(&notes).expect("error while parsing!");
                let file_name = format!("todo-{time}.json");
                let mut file = File::create(&file_name).expect("failed create file!");
                file.write_all(data.as_bytes()).expect("failed to write to file");
                println!("Cheers 🎉, Todo saved to file {file_name}!");
            }
            None => {
                println!("Invalid command!");
            }
        }
        println!("");
    }
}

fn main() {
    println!("📚 Note Taking App!");
    println!("Add `h` for help or to learn more");

    todo_program();
}
