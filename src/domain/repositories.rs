use async_trait::async_trait;

use crate::domain::{
    id::TodoId,
    models::{Todo, TodoInput},
};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn list(&self) -> Vec<Todo>;
    async fn create(&self, todo: TodoInput) -> Todo;
    async fn update(&self, id: TodoId, todo: TodoInput) -> Option<Todo>;
    async fn delete(&self, id: TodoId) -> Option<Todo>;
}
