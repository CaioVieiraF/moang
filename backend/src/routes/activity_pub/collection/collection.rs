use serde::{Deserialize, Serialize};

use crate::routes::activity_pub::{ObjType, Url};

#[derive(Deserialize, Serialize)]
pub struct Collection {
    id: Url,

    #[serde(rename = "type")]
    obj_type: ObjType,

    #[serde(rename = "totalItems")]
    total_items: i32,
}

impl Collection {
    pub fn new(id: Url) -> Collection {
        Collection {
            id,
            obj_type: ObjType::Collection,
            total_items: 0,
        }
    }

    pub fn add_like(&mut self) {
        self.total_items += 1;
    }

    pub fn set_like(&mut self, likes: i32) {
        self.total_items = likes;
    }
}
