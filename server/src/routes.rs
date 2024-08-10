use std::{sync::Arc, vec};

use argon2::{password_hash::rand_core::OsRng, Argon2};
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};

use argon2::{password_hash::SaltString, PasswordHasher};

use serde::{Deserialize, Serialize};

use crate::prisma::PrismaClient;

pub type Database = Extension<Arc<PrismaClient>>;

async fn hello_world(db: Database) -> Response {
    let users = db.user().find_many(vec![]).exec().await.unwrap();

    Json(users).into_response()
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

async fn register(db: Database, Json(body): Json<CreateUser>) -> Response<Body> {
    let password = body.password;

    let argon2 = Argon2::default();
    let salt: SaltString = SaltString::generate(&mut OsRng);

    let hashed_password = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let user = db
        .user()
        .create(
            body.first_name,
            body.last_name,
            body.email,
            hashed_password.to_string(),
            vec![],
        )
        .exec()
        .await;

    Json(user.ok()).into_response()
}

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/user/register", post(register))
}
