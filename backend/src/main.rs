use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, App, HttpServer};
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
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookie_key.clone(),
            ))
            .service(router())
    })
    .bind((blog_address.as_str(), blog_port))?
    .run()
    .await
}
