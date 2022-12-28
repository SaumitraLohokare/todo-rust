use std::fmt::{self, Formatter};

pub struct TodoItem {
    name: String,
    desc: String,
    completed: bool,
}

pub struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoItem {
    pub fn new(name: String, desc: String, completed: bool) -> TodoItem {
        TodoItem {
            name,
            desc,
            completed,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tick = if self.completed { 'X' } else { ' ' };
        write!(f, "[{}]\t{}: \t\t{}", tick, self.name, self.desc)
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    pub fn add_item(&mut self, item: TodoItem) {
        self.list.push(item);
    }

    pub fn remove_item(&mut self, item_name: String) {
        for (i, item) in self.list.iter().enumerate() {
            if item.name == item_name {
                self.list.remove(i);
                return;
            }
        }
    }

    pub fn toggle_completed(&mut self, item_name: String) {
        for item in self.list.iter_mut() {
            if item.name == item_name {
                item.completed = !item.completed;
                break;
            }
        }
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.list.is_empty() {
            write!(f, "<Empty>\n")?;
        }

        for item in self.list.iter() {
            write!(f, "{}\n", item)?;
        }
        fmt::Result::Ok(())
    }
}
