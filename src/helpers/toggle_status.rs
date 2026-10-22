use crate::types::{Status, Todo};

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