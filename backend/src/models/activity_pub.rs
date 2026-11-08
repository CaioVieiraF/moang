use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::actor_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActorKey {
    pub user_id: String,
    pub public_key_pem: String,
    pub private_key_pem: String,
    pub created_at: SystemTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::remote_actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RemoteActor {
    pub id: String,
    pub preferred_username: String,
    pub domain: String,
    pub name: String,
    pub summary: String,
    pub avatar_url: String,
    pub inbox_url: String,
    pub shared_inbox_url: Option<String>,
    pub public_key_id: String,
    pub public_key_pem: String,
    pub fetched_at: SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::followers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follower {
    pub id: i32,
    pub user_id: String,
    pub actor_id: String,
    pub accepted: bool,
    pub created_at: SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::following)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Following {
    pub id: i32,
    pub user_id: String,
    pub actor_id: String,
    pub accepted: bool,
    pub created_at: SystemTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Activity {
    pub id: String,
    pub user_id: Option<String>,
    pub actor_id: String,
    pub activity_type: String,
    pub object_id: Option<String>,
    pub direction: String,
    pub payload: Value,
    pub created_at: SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::deliveries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Delivery {
    pub id: i32,
    pub activity_id: String,
    pub inbox_url: String,
    pub state: String,
    pub attempts: i32,
    pub next_attempt_at: SystemTime,
    pub last_error: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::deliveries)]
pub struct NewDelivery {
    pub activity_id: String,
    pub inbox_url: String,
    pub state: String,
    pub attempts: i32,
    pub next_attempt_at: SystemTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::remote_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RemotePost {
    pub id: String,
    pub actor_id: String,
    pub content: String,
    pub summary: Option<String>,
    pub published: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub url: Option<String>,
    pub in_reply_to: Option<String>,
    pub sensitive: bool,
    pub created_at: SystemTime,
    pub likes_count: i32,
    pub reposts_count: i32,
    pub replies_count: i32,
    pub to_addresses: Value,
    pub cc_addresses: Value,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::remote_interactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RemoteInteraction {
    pub id: String,
    pub actor_id: String,
    pub object_id: String,
    pub interaction_type: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::remote_interactions)]
pub struct NewRemoteInteraction {
    pub id: String,
    pub actor_id: String,
    pub object_id: String,
    pub interaction_type: String,
}
