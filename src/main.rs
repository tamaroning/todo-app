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

impl Todo {
    fn new(name: String) -> Self {
        Todo {
            name,
            is_done: false,
        }
    }
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
async fn add(state: Data<AppState>, mut body: Payload) -> Result<HttpResponse, Error> {
    let mut data = state.todos.lock().unwrap();

    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let name = std::str::from_utf8(&bytes)?;
    data.push(Todo::new(name.to_string()));

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
