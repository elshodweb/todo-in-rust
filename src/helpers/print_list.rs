use crate::types::{Status, Todo};


pub fn print_list(todo_list:&Vec<Todo>){
    println!("Task list:\n");

                if todo_list.is_empty() {
                    println!("Empty");
                }

                for task in todo_list {
                    let status: &str = match task.status {
                        Status::Done => "[✓]",
                        Status::Pending => "[ ]",
                    };
                    println!("{} ID:{} - {}", status, task.id, task.title);
                }

                print!("\n\n");
}