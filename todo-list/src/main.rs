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

use std::io;

trait TodoAction {
    fn add(&mut self, description: &str) -> bool;
}

#[derive(Debug)]

struct TodoItem {
    description: String,
    completed: bool,
    id: u64,
}

#[derive(Debug)]
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

fn print_help() {
    println!("➡");
    println!("");
    println!("Ref details to use with app!");
    println!();
    println!("📝  **Add New Todo**        : Press `i`");
    println!("✏️  **Edit Todo**           : Press `e`");
    println!("🗑️  **Remove Todo**         : Press `d`");
    println!("📋  **List Todos**          : Press `p`");
    println!("ℹ️  **Help Todo**           : Press `h`");
    println!("🔚  **Quit Todo**           : Press `q`");
    println!();
    println!("---------------------------------------");
    println!("🎉 **Welcome to Your Todo App!** 🎉");
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

fn main() {
    println!("📚 Note Taking App!");
    println!("Add `h` for help or to learn more");

    loop {
        let mut user_inupt = String::new();
        io::stdin()
            .read_line(&mut user_inupt)
            .expect("Failed to read input");

        let input = user_inupt.trim();

        match TodoCommand::from_input(input) {
            Some(TodoCommand::Add) => {
                println!("TodoCommand::Add");
            }
            Some(TodoCommand::Edit) => {
                println!("TodoCommand::Edit");
            }
            Some(TodoCommand::Remove) => {
                println!("TodoCommand::Remove");
            }
            Some(TodoCommand::List) => {
                println!("TodoCommand::List");
            }
            Some(TodoCommand::Help) => {
                print_help();
            }
            Some(TodoCommand::Quit) => {
                println!("TodoCommand::Quit");
                break;
            }
            Some(TodoCommand::Save) => {
                println!("TodoCommand::Save");
            }
            None => {
                println!("Invalid command!");
            }
        }
    }

}
