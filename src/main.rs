use actix_web::{App, HttpServer};
use blog::routes::router;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let blog_address = env::var("BLOG_ADDRESS").expect("Variável de ambiente faltando!");
    let blog_port = env::var("BLOG_PORT").expect("Variável de ambiente faltando!");
    let blog_port = blog_port
        .parse::<u16>()
        .expect("Variável de ambiente inválida!");
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    tls_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    tls_builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(router()))
        .bind_openssl((blog_address.as_str(), blog_port), tls_builder)?
        .run()
        .await
}
