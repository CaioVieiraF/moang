use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, http, App, HttpServer};
use blog::routes::router;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cookie_key = Key::from(
        env::var("ACTIX_COOKIE_KEY")
            .expect("ACTIX_COOKIE_KEY must be set!")
            .as_bytes(),
    );

    let blog_address = env::var("BLOG_ADDRESS").expect("Variável de ambiente faltando!");
    let blog_port = env::var("BLOG_PORT").expect("Variável de ambiente faltando!");
    let blog_port = blog_port
        .parse::<u16>()
        .expect("Variável de ambiente inválida!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://moang.com.br")
            .allowed_origin("https://dev.moang.com.br")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookie_key.clone(),
            ))
            .wrap(cors)
            .service(router())
    })
    .bind((blog_address.as_str(), blog_port))?
    .run()
    .await
}
