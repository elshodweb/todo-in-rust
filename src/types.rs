#[derive(PartialEq)]
pub enum Status {
    Done,
    Pending,
}
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub status: Status,
}

pub fn create_todo(id: u32, title: String) -> Todo {
    Todo {
        id,
        title,
        status: Status::Pending,
    }
}
