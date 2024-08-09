use axum::{
    http::{HeaderValue, Method},
    Extension, Router,
};
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use prisma::PrismaClient;

use dotenv::dotenv;

#[allow(warnings, unused)]
mod prisma;
mod routes;

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

    let client = Arc::new(PrismaClient::_builder().build().await.unwrap());

    let app = Router::new()
        .nest("/api", routes::create_route())
        .layer(Extension(client))
        .layer(cors);

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();

    println!("Server listening at http://{host}:{port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
