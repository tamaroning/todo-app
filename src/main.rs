use actix_web::{get, web, App, HttpServer};

struct AppState {
    app_name: String,
    todos: Vec<Todo>,
}

struct Todo {
    name: String,
    is_done: bool,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: "Todo App".to_string(),
        todos: Vec::new(),
    });

    HttpServer::new(move || App::new().app_data(state.clone()).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
