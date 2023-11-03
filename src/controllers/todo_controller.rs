use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};

use crate::{
    models::todo::Todo,
    services::todo_service::{self, TodoService},
};

#[post("/")]
pub async fn create_todo(todo_service: Data<TodoService>, new_todo: Json<Todo>) -> impl Responder {
    println!("{:?}", new_todo);
    let result = todo_service.create_todo(new_todo.0);

    println!("{:?}", result);

    web::Json(result)
}

#[get("/")]
pub async fn get_todos(todo_service: Data<TodoService>) -> impl Responder {
    HttpResponse::Ok().json(todo_service.get_todos())
}

#[get("/{id}")]
pub async fn get_todo_by_id(
    todo_service: Data<TodoService>,
    id: web::Path<String>,
) -> impl Responder {
    let todo = todo_service.get_todo_by_id(id.to_string());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found."),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id),
    );
}
