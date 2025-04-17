use std::sync::Arc;

use crate::domain::{
    error::TodoError,
    id::TodoId,
    models::{Todo, TodoInput},
    repositories::TodoRepository,
};

pub struct TodoService {
    repo: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_todos(&self) -> Vec<Todo> {
        self.repo.list().await
    }

    pub async fn add_todo(&self, todo: TodoInput) -> Todo {
        self.repo.create(todo).await
    }

    pub async fn update_todo(&self, id: TodoId, input: TodoInput) -> Result<Todo, TodoError> {
        self.repo.update(id, input).await.ok_or(TodoError::NotFound)
    }

    pub async fn delete_todo(&self, id: TodoId) -> Result<Todo, TodoError> {
        self.repo.delete(id).await.ok_or(TodoError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::TodoInput;
    use crate::infra::in_memory_repository::InMemoryTodoRepository;

    #[tokio::test]
    async fn service_flow_should_work() {
        // Inject in-memory repository
        let repo = Arc::new(InMemoryTodoRepository::new());
        let service = Arc::new(TodoService::new(repo));

        // Add a todo
        let input = TodoInput {
            title: "Service Test Task".into(),
        };
        let todo = service.add_todo(input.clone()).await;
        assert_eq!(todo.title, input.title);

        // Update the todo
        let updated_option = service
            .update_todo(
                todo.id,
                TodoInput {
                    title: "Updated Service Task".into(),
                },
            )
            .await;
        assert!(updated_option.is_ok());
        assert_eq!(updated_option.unwrap().title, "Updated Service Task");

        // Delete the todo
        let deleted = service.delete_todo(todo.id).await;
        assert!(deleted.is_ok());
        assert_eq!(deleted.unwrap().title, "Updated Service Task");
    }
}
