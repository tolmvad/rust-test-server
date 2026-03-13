use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

// a basic handler function that returns a static string
async fn hello_world() -> &'static str {
    "Hello, world!\n"
}

#[tokio::main]
async fn main() {
    // initialize tracing for logging
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `hello_world` handler
        .route("/", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}