// app/user/login.rs

use actix_web::{HttpResponse, http::StatusCode};
use sqlx::{PgPool, query_as};
use sqlx::Error as SqlxError;
use bcrypt::{verify, DEFAULT_COST};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData<T: Serialize> {
    code: u16,
    status: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> ResponseData<T> {
    fn new(code: StatusCode, message: &str, data: Option<T>) -> Self {
        ResponseData {
            code: code.as_u16(),
            status: code.canonical_reason().unwrap_or("UNKNOWN").to_string(),
            message: message.to_string(),
            data,
        }
    }
}

pub async fn login(pool: &PgPool, req_body: LoginRequest) -> Result<ResponseData, SqlxError> {
    let LoginRequest { username, password } = req_body;

    let user = query_as!(
        User,
        r#"SELECT username, password_hash FROM users WHERE username = $1"#,
        username
    )
    .fetch_optional(pool)
    .await?;

    match user {
        Some(user) => {
            if verify(&password, &user.password_hash).unwrap_or(false) {
                Ok(ResponseData::new(StatusCode::OK, "Login successful"))
            } else {
                Ok(ResponseData::new(StatusCode::UNAUTHORIZED, "Login unsuccessful"))
            }
        }
        None => Ok(ResponseData::new(StatusCode::UNAUTHORIZED, "Login unsuccessful")),
    }
}
