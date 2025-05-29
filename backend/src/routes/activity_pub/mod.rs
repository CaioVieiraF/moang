use std::str::FromStr;

use actix_web::http::{uri::InvalidUri, Uri};
use serde::{Deserialize, Serialize};

pub mod actor;
pub mod collection;
pub mod content_object;

#[derive(Deserialize, Serialize, Clone)]
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
    OrderedCollectionPage,
}
