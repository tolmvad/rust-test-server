use std::net::SocketAddr;
use tokio::runtime::Builder;
use axum::{ Json, Router, routing::get };
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
async fn server_json() -> Result<Json<JsonResponse>, axum::http::StatusCode> {
    Ok(Json(JsonResponse {
        message: "Message to JSON Response",
    }))
}

async fn run() -> Result<()> {
    // Создание маршрутов
    let app = Router::new()
        .route("/", get(server_welcome))
        .route("/json", get(server_json));

    // Задаём адрес и порт сервера
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    // Запуск сервера
    let listener = std::net::TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn main() -> Result<()> {
    let rt = Builder::new_multi_thread().enable_all().build()?;
    rt.block_on( async { run().await } )?;

    Ok(())
}
