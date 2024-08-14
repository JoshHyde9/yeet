use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: i64,
}

pub fn get_jwt(user_id: String) -> Result<String, String> {
    let token = encode(
        &Header::default(),
        &Claims {
            id: user_id,
            exp: (Utc::now() + Duration::hours(8)).timestamp(),
        },
        &EncodingKey::from_secret(
            env::var("JWT_TOKEN")
                .expect("Error: JWT_TOKEN env not found")
                .as_bytes(),
        ),
    )
    .map_err(|e| e.to_string());

    return token;
}
