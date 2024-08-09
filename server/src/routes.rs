use std::{sync::Arc, vec};

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};

use crate::prisma::PrismaClient;

pub type Database = Extension<Arc<PrismaClient>>;

async fn hello_world(db: Database) -> Response {
    let users = db.user().find_many(vec![]).exec().await.unwrap();

    Json(users).into_response()
}

pub fn create_route() -> Router {
    Router::new().route("/", get(hello_world))
}
