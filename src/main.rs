use actix_web::web::{self, service};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use todo_api::controllers;
use todo_api::repositories;
use todo_api::services;

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("initial controller.");

    let database_mockup = repositories::database_mockup::Database::new();
    let todo_service = services::todo_service::TodoService::new(database_mockup);

    let todo_data_service = web::Data::new(todo_service);

    HttpServer::new(move || {
        App::new()
            .app_data(todo_data_service.clone())
            .configure(controllers::todo_controller::config)
            .service(health_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
