mod activity_pub;
mod post;
mod user;

use serde::{Deserialize, Serialize};

pub use activity_pub::{
    Activity, ActorKey, Delivery, Follower, Following, NewDelivery, NewRemoteInteraction,
    RemoteActor, RemoteInteraction, RemotePost,
};
pub use post::{GetPost, NewPost, Post};
pub use user::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub iat: usize,
}
