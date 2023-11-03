use actix_web::{
    post,
    web::{self, Data, Json},
    Responder,
};

use crate::{models::todo::Todo, services::todo_service::TodoService};

#[post("/")]
pub async fn create_todo(todo_service: Data<TodoService>, new_todo: Json<Todo>) -> impl Responder {
    println!("{:?}", new_todo);
    let result = todo_service.create_todo(new_todo.0);

    println!("{:?}", result);

    web::Json(result)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todo").service(create_todo));
}
