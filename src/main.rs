use actix_web::{
    get, post,
    web::{BytesMut, Data, Json, Payload},
    App, Error, HttpResponse, HttpServer,
};
#[allow(unused)]
use futures_core::stream::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use std::future::Future;
use std::sync::Mutex;

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Todo {
    name: String,
    is_done: bool,
}

#[get("/")]
async fn index() -> HttpResponse {
    let html = include_str!("../index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/api/todos")]
async fn todos(state: Data<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    Json(todos.to_vec())
}

#[post("/api/add")]
async fn add(state: Data<AppState>, body: Json<Todo>) -> Result<HttpResponse, Error> {
    let mut data = state.todos.lock().unwrap();
    // implicit deref: Json<Todo> -> &Todo
    data.push(body.clone());

    Ok(HttpResponse::NoContent().finish())
}

#[get("/api/reset")]
async fn reset(state: Data<AppState>) -> HttpResponse {
    let mut data = state.todos.lock().unwrap();
    *data = vec![];

    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Data::new(AppState {
        todos: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(todos)
            .service(add)
            .service(reset)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
