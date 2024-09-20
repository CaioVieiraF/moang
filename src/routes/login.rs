use std::env;

use crate::establish_connection;
use actix_web::{get, web::Json, HttpResponse};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: String,
    iat: usize,
}

#[derive(Serialize, Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    token: String,
}

fn generate_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let secret = env::var("JWT_HASH").expect("Variable must be set!");
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

fn is_password_valid(hashed_password: String, password_to_verify: String) -> Option<bool> {
    match argon2::verify_encoded(&hashed_password, password_to_verify.as_bytes()) {
        Ok(is_valid) => Some(is_valid),
        Err(_) => None,
    }
}

#[get("/login")]
async fn login(request_data: Json<LoginData>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let login_data = request_data.into_inner();

    let connection = &mut establish_connection();
    let query_result = users
        .filter(email.eq(&login_data.email))
        .select(password)
        .first::<String>(connection)
        .optional();

    match query_result {
        Ok(Some(user_password)) => {
            if let Some(is_valid) = is_password_valid(user_password, login_data.password) {
                if is_valid {
                    let my_claims = Claims {
                        sub: login_data.email,
                        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
                        iat: chrono::Utc::now().timestamp() as usize,
                    };
                    let generate_token = generate_token(my_claims);
                    match generate_token {
                        Ok(token) => {
                            let response = SuccessResponse { token };
                            HttpResponse::Ok().json(response)
                        }
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                } else {
                    HttpResponse::Unauthorized().finish()
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
