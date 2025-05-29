use std::str::FromStr;

use actix_web::{
    http::{uri::InvalidUri, Uri},
    web, Scope,
};
use serde::{Deserialize, Serialize};

pub mod actor;
pub mod collection;
pub mod post;

#[derive(Deserialize, Serialize)]
pub struct Url(String);

impl TryFrom<String> for Url {
    type Error = InvalidUri;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _ = Uri::from_str(&value)?;
        Ok(Url(value))
    }
}

#[derive(Deserialize, Serialize)]
pub enum ObjType {
    Note,
    Person,
    Create,
    OrderedCollection,
}

pub fn activity_pub_routes() -> Scope {
    web::scope("fed").service(actor::get_actor)
}
