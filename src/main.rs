
use std::io;
use crate::command_input::command_input;
pub mod command_input;
pub mod extractor;

fn main() -> io::Result<()>{
    println!("TODO list APP!");
    println!("Your name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    println!("Thanks {}", name.to_uppercase());
    println!("Available commands:");
    println!("1.Ads new task/s => add [task];");
    println!("2.Lists all tasks => list;");
    println!("3.Toggles the status of a task (Done/Pending) => toggle [ID]");
    println!("4.Removes a task => rm [ID];");
    println!("5.Deletes all tasks => reset");
    println!("6.Quite => quite\n\n");
    
    command_input()?;
    
    Ok(())
}