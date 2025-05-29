use serde::{Deserialize, Serialize};

use super::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Collection<U = Url> {
    id: U,
    first: U,
    last: U,

    #[serde(rename = "@context")]
    context: U,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i32,
}
