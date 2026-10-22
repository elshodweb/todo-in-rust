use crate::types::Todo;

pub fn rm_todo(id: u32, todo_list: &mut Vec<Todo>) {
    if let Some(index) = todo_list.iter().position(|item| item.id == id) {
        todo_list.swap_remove(index);
    }
}