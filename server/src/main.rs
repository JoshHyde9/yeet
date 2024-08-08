use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

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
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

    let app = Router::new().route("/", get(hello_world).layer(cors));

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
