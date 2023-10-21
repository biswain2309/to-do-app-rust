pub struct Todo {
    pub description: String,
    pub status: bool,
}

pub fn add_item(description: String) -> Todo {
    Todo {
        description,
        status: false,
    }
}

fn main() {
    let mut todos: Vec<T> = Vec::new();
}
