use crate::domain::id::TodoId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn update(&mut self, input: TodoInput) {
        self.title = input.title;
    }
}

#[derive(Deserialize, Clone)]
pub struct TodoInput {
    pub title: String,
}

impl From<TodoInput> for Todo {
    fn from(input: TodoInput) -> Self {
        Todo {
            id: TodoId::new(),
            title: input.title,
            completed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn todo_input_converts_to_todo() {
        let input = TodoInput {
            title: "Buy milk".into(),
        };
        let todo: Todo = input.into();
        assert_eq!(todo.title, "Buy milk");
        assert_eq!(todo.completed, false);
    }
}
