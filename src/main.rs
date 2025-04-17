use actix_web::{App, HttpResponse, HttpServer, Responder, error, get, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;
use envconfig::Envconfig;
use std::sync::Arc;

use todo::{
    app::services::TodoService,
    infra::in_memory_repository::InMemoryTodoRepository,
    interfaces::{config::Config, handlers},
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::init_from_env().expect("Failed to load config");
    env_logger::Builder::from_env(Env::default().default_filter_or(&config.log_level)).init();

    let repo = Arc::new(InMemoryTodoRepository::new());
    let svc = Arc::new(TodoService::new(repo));
    let state = web::Data::new(handlers::AppState { service: svc });

    println!("Starting server at http://127.0.0.1:{}", config.port);
    HttpServer::new(move || {
        let bad_request =
            web::PathConfig::default().error_handler(|e, _| error::ErrorBadRequest(e));
        App::new()
            .wrap(Logger::default())
            .app_data(bad_request)
            .app_data(state.clone())
            .service(index)
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let service = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&service, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let status: String = test::read_body_json(resp).await;
        assert_eq!(status, "OK");
    }
}
