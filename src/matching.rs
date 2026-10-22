use crate::helpers::{ command_extractor::extract_command, id_ectractor::extract_id, print_list::print_list, remove_todo::rm_todo, toggle_status::toggle,};
use crate::{ types::{Todo, create_todo}};
use std::{io};


pub fn matching() -> Result<(), io::Error> {
    let mut command = String::new();
    let mut todo_list: Vec<Todo> = Vec::new();
    let mut counter: u32 = 0;

    while ["quite","quit"].contains(&command.trim()) {
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
                print_list(&todo_list);
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

                rm_todo(id, &mut todo_list);
            }
            "reset" => {
                todo_list.clear();
            }
            "quite" => {
                break;
            }
            "quit" => {
                break;
            }
            _ => {
                println!("unavailable command!!!");
            }
        }
    }

    Ok(())
}





