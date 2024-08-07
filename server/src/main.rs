use axum::{routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn hello_world() -> Json<Response> {
    let response = Response {
        message: "Hello World!".to_owned(),
    };

    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let address = "localhost";
    let port = 5000;

    let listener = TcpListener::bind(format!("{}:{}", address, port.to_string()))
        .await
        .unwrap();

    println!(
        "Server listening at http://{}:{}",
        address,
        port.to_string()
    );

    axum::serve(listener, app).await.unwrap();
}
