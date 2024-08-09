use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use dotenv::dotenv;
use std::env;

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
    dotenv().ok();

    let host = env::var("HOST").expect("Error: host env not found");
    let port = env::var("PORT").expect("Error: port env not found");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(
            format!("http://{host}:{}", 3000)
                .parse::<HeaderValue>()
                .unwrap(),
        );

    let app = Router::new().route("/", get(hello_world).layer(cors));

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();

    println!("Server listening at http://{host}:{port}");

    axum::serve(listener, app).await.unwrap();
}
