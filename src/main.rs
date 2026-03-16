use axum::{
    Json,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde::Serialize;

// Обработчик для корня
async fn server_welcome() -> &'static str {
    "Welcome to the Rust test server!
        Rust + Axum + Tokio\n"
}

#[derive(Serialize)]
struct JsonResponse {
    message: &'static str,
}

// Обработчик для вызова /json
async fn server_json() -> Result<Json<JsonResponse>, axum::http::StatusCode> {
    Ok(Json(JsonResponse {
        message: "Message to JSON Response",
    }))
}

#[tokio::main]
async fn main() {

    // Создание маршрутов
    let app = Router::new()
        // Переходы к обработчикам GET запросов
        .route("/", get(server_welcome))
        .route("/json", get(server_json));

    // Задаём адрес и порт сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Запуск сервера
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
