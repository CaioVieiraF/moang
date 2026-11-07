mod activity_pub;
mod post;
mod user;

use serde::{Deserialize, Serialize};

pub use activity_pub::{
    ActorKey, Activity, Delivery, Follower, Following, NewDelivery, RemoteActor, RemotePost,
};
pub use post::{GetPost, NewPost, Post};
pub use user::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub iat: usize,
}
