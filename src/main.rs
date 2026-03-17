use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::{
    Json, Router,
    routing::get,
};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct JsonResponse {
    message: &'static str,
}

// Обработчик для корня
async fn server_welcome() -> &'static str {
    "Welcome to the Rust test server!
        Rust + Axum + Tokio\n"
}

// Обработчик для вызова /json
async fn server_json() -> Json<JsonResponse> {
    Json(JsonResponse {
        message: "Message to JSON Response",
    })
}

async fn run() -> Result<()> {
    // Создание маршрутов
    let app = Router::new()
        .route("/", get(server_welcome))
        .route("/json", get(server_json));

    // Задаём адрес и порт сервера
    const ADDRESS: &str = "127.0.0.1:3000";
    let addr: SocketAddr = ADDRESS.parse()?;
    println!("Listening on http://{}", addr);

    // Запуск сервера
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
