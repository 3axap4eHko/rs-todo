use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{
    id::TodoId,
    models::{Todo, TodoInput},
    repositories::TodoRepository,
};

pub struct InMemoryTodoRepository {
    store: RwLock<HashMap<TodoId, Todo>>,
}

impl Default for InMemoryTodoRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn list(&self) -> Vec<Todo> {
        let guard = self.store.read().await;
        guard.values().cloned().collect()
    }

    async fn get(&self, id: TodoId) -> Option<Todo> {
        let guard = self.store.read().await;
        // should be cloned
        guard.get(&id).cloned()
    }

    async fn create(&self, input: TodoInput) -> Todo {
        let mut guard = self.store.write().await;
        let todo: Todo = input.into();
        guard.insert(todo.id, todo.clone());
        todo
    }

    async fn update(&self, id: TodoId, input: TodoInput) -> Option<Todo> {
        let mut guard = self.store.write().await;
        guard.get_mut(&id).map(|t| {
            t.update(input);
            t.clone()
        })
    }

    async fn delete(&self, id: TodoId) -> Option<Todo> {
        let mut guard = self.store.write().await;
        guard.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::TodoInput;

    #[tokio::test]
    async fn test_repository_crud_operations() {
        let repo = InMemoryTodoRepository::new();

        // Create
        let input = TodoInput {
            title: "Task 1".into(),
        };
        let created_todo = repo.create(input.clone()).await;
        assert_eq!(created_todo.title, "Task 1");

        // List
        let todos = repo.list().await;
        assert_eq!(todos.len(), 1);

        // Update
        let updated_option = repo
            .update(
                created_todo.id,
                TodoInput {
                    title: "Updated Task".into(),
                },
            )
            .await;
        assert!(updated_option.is_some());
        let updated_todo = updated_option.unwrap();
        assert_eq!(updated_todo.title, "Updated Task");

        // Delete
        let deleted_option = repo.delete(created_todo.id).await;
        assert!(deleted_option.is_some());
        let deleted_todo = deleted_option.unwrap();
        assert_eq!(deleted_todo.title, "Updated Task");

        let todos_after_deletion = repo.list().await;
        assert_eq!(todos_after_deletion.len(), 0);
    }
}
