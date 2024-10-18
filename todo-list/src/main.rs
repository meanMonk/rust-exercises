
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

trait TodoAction {
    fn add(&mut self,description: &str) -> bool;
}

#[derive(Debug)]

struct TodoItem {
    description: String, 
    completed: bool, 
    id: u64
}

#[derive(Debug)]
struct Todo {
    list: Vec<TodoItem>,
}

// prepare constrcutor for Todo.

impl Todo {
    // function to create new todo
    fn new() -> Self {
        Todo {
            list: Vec::new()
        }
    }
    
    // function to list todo data.
    fn list(&self) {
        for todo in &self.list {
            let status = if todo.completed {'✅'} else {'❎'};
            println!("{status} {}", todo.description);
            println!("");
        }
    }
}

impl TodoAction for Todo {
    fn add(&mut self, description: &str) -> bool {
        let _todo =  TodoItem {
            description: description.to_string(), 
            completed: false, 
            id: self.list.len() as u64 + 1
        };
        
        self.list.push(_todo);
        
        true
    }
}


fn main() {
    println!("📚 Note Taking App!");
    
    let mut notes = Todo::new();
    
    notes.add("Lot's of things to finish!");
    
    println!("well done todo updated");
    
    println!("📚 ----");
    notes.list();
    
}