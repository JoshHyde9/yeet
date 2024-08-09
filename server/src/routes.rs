use std::{sync::Arc, vec};

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use serde::Serialize;

use crate::prisma::PrismaClient;

pub type Database = Extension<Arc<PrismaClient>>;

#[derive(Serialize)]
pub struct Data {
    message: String,
}

async fn hello_world(db: Database) -> Response {
    let users = db.user().find_many(vec![]).exec().await.unwrap();

    Json(users).into_response()
}

pub fn create_route() -> Router {
    Router::new().route("/", get(hello_world))
}
