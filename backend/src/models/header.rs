use http_signature_normalization::Config;
use std::{env, fmt::Display, time::SystemTime};

use actix_web::http::header::HeaderValue;
use base64ct::{Base64, Encoding};
use dotenv::dotenv;

use crate::routes::activity_pub::Url;

#[derive(Debug)]
pub struct Signature(String);

impl Signature {
    pub fn new() -> Signature {
        Signature(String::new())
    }
}

impl From<Signature> for HeaderValue {
    fn from(value: Signature) -> Self {
        let header = value.0;
        println!("{header}");
        HeaderValue::from_str(&header.as_str()).unwrap()
    }
}
