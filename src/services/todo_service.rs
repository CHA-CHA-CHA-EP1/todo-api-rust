use actix_web::HttpResponse;

use crate::models::todo::Todo;
use crate::repositories::database_mockup::Database;

#[derive(Clone)]
pub struct TodoService {
    pub databse_mock: Database,
}

impl TodoService {
    pub fn new(database_mockup: Database) -> Self {
        Self {
            databse_mock: database_mockup,
        }
    }

    pub fn create_todo(&self, new_todo: Todo) -> Todo {
        println!("create todo service.");
        let todo = self.databse_mock.create_todo(new_todo).unwrap();
        todo
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.databse_mock.get_todos();
        todos
    }

    pub fn get_todo_by_id(&self, id: String) -> Option<Todo> {
        self.databse_mock.get_todo_by_id(&id)
    }
}
