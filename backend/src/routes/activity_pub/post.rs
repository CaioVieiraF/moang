use serde::{Deserialize, Serialize};

use super::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Post {
    #[serde(rename = "@context")]
    context: Url,
    id: Url,
    #[serde(rename = "type")]
    obj_type: ObjType,
    actor: Url,
}
