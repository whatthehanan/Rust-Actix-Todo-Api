use std::sync::Mutex;

use actix_web::{get, main, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

struct AppState {
    app_name: String,
    todos: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoPayload {
    completed: bool,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("{}", app_name)
}

async fn get_todos(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let data = state.lock().unwrap();
    HttpResponse::Ok().json(&data.todos)
}

async fn add_todo(req_body: web::Json<Todo>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let mut data = state.lock().unwrap();
    data.todos.push(req_body.into_inner());

    HttpResponse::Created().finish()
}

async fn delete_todo(path: web::Path<i32>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let mut data = state.lock().unwrap();
    let todo_id = path.into_inner();

    let todo_to_delete_index = data.todos.iter().position(|x| x.id == todo_id).unwrap();
    data.todos.remove(todo_to_delete_index);

    HttpResponse::Ok().finish()
}

async fn update_todo(
    path: web::Path<i32>,
    req_body: web::Json<TodoPayload>,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let mut data = state.lock().unwrap();
    let todo_id = path.into_inner();

    let todo_to_update_index = data.todos.iter().position(|x| x.id == todo_id).unwrap();
    data.todos[todo_to_update_index].completed = req_body.into_inner().completed;

    HttpResponse::Ok().finish()
}

async fn get_todo_by_id(path: web::Path<i32>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let data = state.lock().unwrap();
    let todo_id = path.into_inner();

    let todo_index = data.todos.iter().position(|x| x.id == todo_id).unwrap();

    HttpResponse::Ok().json(&data.todos[todo_index])
}

#[main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(AppState {
        app_name: String::from("Rust Todo API"),
        todos: (vec![].clone()),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&app_state))
            .service(hello)
            .service(
                web::scope("/api").service(
                    web::scope("/todo")
                        .service(
                            web::resource("")
                                .route(web::get().to(get_todos))
                                .route(web::post().to(add_todo)),
                        )
                        .service(
                            web::resource("/{todo_id}")
                                .route(web::delete().to(delete_todo))
                                .route(web::put().to(update_todo))
                                .route(web::get().to(get_todo_by_id)),
                        ),
                ),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
