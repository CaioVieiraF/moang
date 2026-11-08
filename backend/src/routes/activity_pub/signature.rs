use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime},
};

use actix_web::http::header::HeaderMap;
use base64ct::{Base64, Encoding};
use http_signature_normalization::{
    verify::{Algorithm, DeprecatedAlgorithm},
    Config,
};
use openssl::{
    hash::MessageDigest,
    pkey::PKey,
    sign::{Signer, Verifier},
};
use reqwest::header::{
    HeaderMap as ReqwestHeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, DATE, HOST, USER_AGENT,
};
use sha2::{Digest, Sha256};
use url::Url;

use crate::{
    config::AppConfig,
    models::{ActorKey, RemoteActor},
};

pub const SIGNATURE_HEADER: &str = "signature";
const SIGNED_HEADERS: &str = "(request-target) host date digest content-type";
const SIGNED_GET_HEADERS: &str = "(request-target) host date";

/// Sign a GET request.
///
/// Some instances (including mastodon.social) use authorized fetch and refuse
/// to serve actors or objects to unsigned requests.
pub fn sign_get_request(
    url: &Url,
    key: &ActorKey,
    key_id: &str,
) -> Result<ReqwestHeaderMap, String> {
    let host = host_header(url)?;
    let path = path_and_query(url);
    let date = httpdate::fmt_http_date(SystemTime::now());

    let signing_string = format!(
        "(request-target): get {}\nhost: {}\ndate: {}",
        path, host, date
    );
    let signature = rsa_sign(&key.private_key_pem, &signing_string)?;
    let signature_header = format!(
        "keyId=\"{key_id}\",algorithm=\"rsa-sha256\",headers=\"{SIGNED_GET_HEADERS}\",signature=\"{signature}\""
    );

    let mut headers = ReqwestHeaderMap::new();
    headers.insert(HOST, HeaderValue::from_str(&host).map_err(|e| e.to_string())?);
    headers.insert(DATE, HeaderValue::from_str(&date).map_err(|e| e.to_string())?);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(AppConfig::get().user_agent()).map_err(|e| e.to_string())?,
    );
    headers.insert(
        HeaderName::from_static(SIGNATURE_HEADER),
        HeaderValue::from_str(&signature_header).map_err(|e| e.to_string())?,
    );

    Ok(headers)
}

/// Sign a server-to-server POST using RSA-SHA256.
///
/// Mastodon and most fediverse software still expect `algorithm="rsa-sha256"` in
/// the Signature header, so the header is built by hand here.
pub fn sign_request(
    method: &str,
    url: &Url,
    body: &[u8],
    key: &ActorKey,
    key_id: &str,
) -> Result<ReqwestHeaderMap, String> {
    let host = host_header(url)?;
    let path = path_and_query(url);
    let date = httpdate::fmt_http_date(SystemTime::now());
    let digest = digest_header(body);
    let content_type = "application/activity+json";

    let signing_string = format!(
        "(request-target): {} {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        method.to_lowercase(),
        path,
        host,
        date,
        digest,
        content_type
    );

    let signature = rsa_sign(&key.private_key_pem, &signing_string)?;

    let signature_header = format!(
        "keyId=\"{key_id}\",algorithm=\"rsa-sha256\",headers=\"{SIGNED_HEADERS}\",signature=\"{signature}\""
    );

    let mut headers = ReqwestHeaderMap::new();
    headers.insert(HOST, HeaderValue::from_str(&host).map_err(|e| e.to_string())?);
    headers.insert(DATE, HeaderValue::from_str(&date).map_err(|e| e.to_string())?);
    headers.insert(
        HeaderName::from_static("digest"),
        HeaderValue::from_str(&digest).map_err(|e| e.to_string())?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(AppConfig::get().user_agent()).map_err(|e| e.to_string())?,
    );
    headers.insert(
        HeaderName::from_static(SIGNATURE_HEADER),
        HeaderValue::from_str(&signature_header).map_err(|e| e.to_string())?,
    );

    Ok(headers)
}

/// Verify a signed inbox request from a remote actor.
pub fn verify_request(
    method: &str,
    path_and_query: &str,
    headers: &HeaderMap,
    body: &[u8],
    actor: &RemoteActor,
) -> Result<(), String> {
    verify_digest(headers, body)?;

    let mut header_map = BTreeMap::new();
    for (name, value) in headers {
        header_map.insert(
            name.as_str().to_string(),
            value.to_str().map_err(|e| e.to_string())?.to_string(),
        );
    }

    let config = Config::default()
        .set_expiration(Duration::from_secs(300))
        .mastodon_compat()
        .require_digest();

    let unverified = config
        .begin_verify(method, path_and_query, header_map)
        .map_err(|error| error.to_string())?;

    if let Some(algorithm) = unverified.algorithm() {
        let accepted = match algorithm {
            Algorithm::Hs2019 => true,
            Algorithm::Deprecated(DeprecatedAlgorithm::RsaSha256) => true,
            Algorithm::Unknown(name) => name.eq_ignore_ascii_case("rsa-sha256"),
            _ => false,
        };
        if !accepted {
            return Err("unsupported signature algorithm".to_string());
        }
    }

    let key_id = unverified.key_id();
    if key_id != actor.public_key_id
        && key_id != actor.id
        && !key_id.starts_with(&format!("{}#", actor.id))
    {
        return Err(format!(
            "signature key {key_id} does not belong to actor {}",
            actor.id
        ));
    }

    let signature = Base64::decode_vec(unverified.signature())
        .map_err(|error| format!("invalid signature base64: {error}"))?;
    let public_key =
        PKey::public_key_from_pem(actor.public_key_pem.as_bytes()).map_err(|e| e.to_string())?;
    let mut verifier =
        Verifier::new(MessageDigest::sha256(), &public_key).map_err(|e| e.to_string())?;
    verifier
        .update(unverified.signing_string().as_bytes())
        .map_err(|e| e.to_string())?;

    if verifier.verify(&signature).map_err(|e| e.to_string())? {
        Ok(())
    } else {
        Err("invalid HTTP signature".to_string())
    }
}

pub fn verify_digest(headers: &HeaderMap, body: &[u8]) -> Result<(), String> {
    let digest = headers
        .get("digest")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| "missing Digest header".to_string())?;

    let encoded = digest
        .strip_prefix("SHA-256=")
        .or_else(|| digest.strip_prefix("sha-256="))
        .ok_or_else(|| "unsupported Digest algorithm".to_string())?;

    let expected = Base64::decode_vec(encoded)
        .map_err(|error| format!("invalid Digest base64: {error}"))?;
    let actual = Sha256::digest(body);

    if expected.as_slice() == actual.as_slice() {
        Ok(())
    } else {
        Err("Digest does not match request body".to_string())
    }
}

pub fn signature_key_id(headers: &HeaderMap) -> Option<String> {
    let value = headers
        .get(SIGNATURE_HEADER)
        .or_else(|| headers.get("authorization"))?
        .to_str()
        .ok()?;

    for part in value.trim_start_matches("Signature").trim().split(',') {
        let mut pair = part.splitn(2, '=');
        let name = pair.next()?.trim();
        let value = pair.next()?.trim().trim_matches('"');
        if name.eq_ignore_ascii_case("keyId") {
            return Some(value.to_string());
        }
    }

    None
}

pub fn digest_header(body: &[u8]) -> String {
    format!("SHA-256={}", Base64::encode_string(&Sha256::digest(body)))
}

fn rsa_sign(private_key_pem: &str, signing_string: &str) -> Result<String, String> {
    let private_key =
        PKey::private_key_from_pem(private_key_pem.as_bytes()).map_err(|e| e.to_string())?;
    let mut signer = Signer::new(MessageDigest::sha256(), &private_key).map_err(|e| e.to_string())?;
    signer
        .update(signing_string.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(Base64::encode_string(
        &signer.sign_to_vec().map_err(|e| e.to_string())?,
    ))
}

pub fn host_header(url: &Url) -> Result<String, String> {
    let host = url
        .host_str()
        .ok_or_else(|| format!("URL is missing a host: {url}"))?;

    match url.port() {
        Some(port) => Ok(format!("{host}:{port}")),
        None => Ok(host.to_string()),
    }
}

pub fn path_and_query(url: &Url) -> String {
    match url.query() {
        Some(query) => format!("{}?{}", url.path(), query),
        None => url.path().to_string(),
    }
}
