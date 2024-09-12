mod routes;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

use routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let blog_address = env::var("BLOG_ADDRESS").expect("Variável de ambiente faltando!");
    let blog_port = env::var("BLOG_PORT").expect("Variável de ambiente faltando!");
    let blog_port = blog_port
        .parse::<u16>()
        .expect("Variável de ambiente inválida!");

    HttpServer::new(|| App::new().service(router()))
        .bind((blog_address.as_str(), blog_port))?
        .run()
        .await
}
