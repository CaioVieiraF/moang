pub mod header;
mod post;
mod user;

use serde::{Deserialize, Serialize};

pub use post::{GetPost, NewPost, Post};
pub use user::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub iat: usize,
}
