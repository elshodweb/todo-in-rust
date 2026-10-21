use crate::extractor::extract_command;
use std::{fmt::Error, io};

#[derive(PartialEq)]
pub enum Status {
    Done,
    Pending,
}
pub struct Todo {
    id: u32,
    title: String,
    status: Status,
}

pub fn create_todo(id: u32, title: String) -> Todo {
    Todo {
        id,
        title,
        status: Status::Pending,
    }
}

pub fn command_input() -> Result<(), io::Error> {
    let mut command = String::new();
    let mut todo_list: Vec<Todo> = Vec::new();
    let mut counter: u32 = 0;

    while command.trim() != "quite" {
        println!("your new command:");

        command.clear();
        io::stdin().read_line(&mut command)?;

        if command.trim().is_empty() {
            println!("your command is empty!!!");
        }

        let vec_command_and_content = extract_command(&command);
        println!("\nOK!\n\n");

        let key_word = vec_command_and_content[0];

        match key_word {
            "add" => {
                match vec_command_and_content.get(1) {
                    Some(title) if !title.is_empty() =>{
                        todo_list.push(create_todo(counter, title.to_string()));
                        counter += 1;
                    }
                    _ => print!("Please provide a task description!")
                };                
            }
            "list" => {
                println!("Task list:\n");

                if todo_list.is_empty() {
                    println!("Empty");
                }

                for task in &todo_list {
                    let status: &str = match task.status {
                        Status::Done => "[✓]",
                        Status::Pending => "[ ]",
                    };
                    println!("{} ID:{} - {}", status, task.id, task.title);
                }

                print!("\n\n");
            }
            "toggle" => {
                let id = match extract_id(&vec_command_and_content) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                toggle(id, &mut todo_list);
            }
            "rm" => {
                let id = match extract_id(&vec_command_and_content) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                rm(id, &mut todo_list);
            }
            "reset" => {
                todo_list = vec![];
            }
            "quite" => {
                break;
            }
            _ => {
                println!("unavailable command!!!");
            }
        }
    }

    Ok(())
}

pub fn extract_id(vec_command_and_content: &Vec<&str>) -> Result<u32, Error> {
    let raw_id = match vec_command_and_content.get(1) {
        Some(id) => id,
        None => {
            println!("You need to write an ID after rm command!!!");
            return Err(Error);
        }
    };

    let _: u32 = match raw_id.trim().parse() {
        Ok(id) => return Ok(id),
        Err(_) => {
            println!("ID must be a valid number!!!");
            return Err(Error);
        }
    };
}

pub fn rm(id: u32, todo_list: &mut Vec<Todo>) {
    if let Some(index) = todo_list.iter().position(|item| item.id == id) {
        todo_list.swap_remove(index);
    }
}

pub fn toggle(id: u32, todo_list: &mut Vec<Todo>) {
    let task: Option<&mut Todo> = todo_list.iter_mut().find(|x| x.id == id);
    match task {
        None => {
            println!("Not found!")
        }
        Some(num) => {
            num.status = if num.status == Status::Done {
                Status::Pending
            } else {
                Status::Done
            }
        }
    }
}
