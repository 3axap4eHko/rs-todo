use std::sync::Arc;

use crate::{
    app::services::TodoService,
    domain::{error::TodoError, id::TodoId, models::TodoInput},
};
use actix_web::{HttpResponse, delete, get, post, put, web};

pub struct AppState {
    pub service: Arc<TodoService>,
}

#[get("/todos")]
async fn list_todos(data: web::Data<AppState>) -> Result<HttpResponse, TodoError> {
    let todos = data.service.list_todos().await;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn add_todo(
    data: web::Data<AppState>,
    item: web::Json<TodoInput>,
) -> Result<HttpResponse, TodoError> {
    let todo = data.service.add_todo(item.into_inner()).await;
    Ok(HttpResponse::Ok().json(todo))
}

#[put("/todos/{id}")]
async fn update_todo(
    data: web::Data<AppState>,
    id: web::Path<TodoId>,
    item: web::Json<TodoInput>,
) -> Result<HttpResponse, TodoError> {
    let updated = data.service.update_todo(*id, item.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/todos/{id}")]
async fn delete_todo(
    data: web::Data<AppState>,
    id: web::Path<TodoId>,
) -> Result<HttpResponse, TodoError> {
    let deleted = data.service.delete_todo(*id).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_todos)
        .service(add_todo)
        .service(update_todo)
        .service(delete_todo);
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test};

    use super::*;
    use crate::{
        app::services::TodoService, domain::models::Todo,
        infra::in_memory_repository::InMemoryTodoRepository, interfaces::handlers::AppState,
    };

    pub fn init(cfg: &mut web::ServiceConfig) {
        let repo = Arc::new(InMemoryTodoRepository::new());
        let svc = Arc::new(TodoService::new(repo));
        let state = web::Data::new(AppState { service: svc });

        cfg.app_data(state.clone());
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = App::new().configure(init).service(list_todos);
        let service = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/todos").to_request();

        let resp = test::call_service(&service, req).await;
        let status = resp.status();
        let bytes = test::read_body(resp).await; // read body once

        // status assertion prints body on failure
        assert_eq!(
            status,
            StatusCode::OK,
            "unexpected status {}\nbody = {}",
            status,
            std::str::from_utf8(&bytes).unwrap_or("<non‑utf8>")
        );

        // reuse the same bytes for JSON deserialization
        let todos: Vec<Todo> = serde_json::from_slice(&bytes).expect("valid JSON");
        assert!(todos.is_empty());
    }
}
