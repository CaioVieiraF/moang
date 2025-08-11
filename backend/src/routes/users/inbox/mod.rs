mod post_follow;
use std::{collections::BTreeMap, env, sync::Arc, time::Duration};

use actix_web::{http::header::HeaderMap, web, Scope};
use base64ct::{Base64Url, Encoding};
use dotenv::dotenv;
use http_signature_normalization::Config;
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, PKeyRef},
    sign::Verifier,
};

pub fn inbox_router() -> Scope {
    web::scope("inbox").route("", web::post().to(post_follow::follow_actor))
}

fn get_user_public_key() -> Arc<[u8]> {
    dotenv().ok();
    let public_key_pem = env::var("PUBLIC_KEY").expect("PUBLIC_KEY must be set!");
    println!("{public_key_pem}");
    Arc::from(public_key_pem.as_bytes())
}

fn verify_signature(headers: HeaderMap, path: &str) -> bool {
    let public_key_pem = get_user_public_key();
    let public_key = PKey::public_key_from_pem(public_key_pem.as_ref()).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), public_key.as_ref()).unwrap();

    let mut btree_headers = BTreeMap::new();

    for (header, value) in headers {
        btree_headers.insert(header.to_string(), String::from(value.to_str().unwrap()));
    }

    let config = Config::default()
        .set_expiration(Duration::from_secs(30))
        .mastodon_compat()
        .require_digest()
        .begin_verify("POST", path, btree_headers.clone());

    match config {
        Ok(c) => {
            let result = c.verify(|sig, sig_string| {
                let decoded_sig = Base64Url::decode_vec(sig).unwrap();
                println!("{decoded_sig:?}");
                println!("\n{sig_string}");
                verifier.update(&decoded_sig).unwrap();
                verifier.verify(sig_string.as_bytes()).unwrap()
            });

            return result;
        }
        Err(e) => {
            dbg!(e);
            return false;
        }
    }
}
