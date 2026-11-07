use std::time::SystemTime;

use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub summary: String,
    pub avatar_url: String,
    pub created_at: SystemTime,
    pub is_locked: bool,
    pub is_discoverable: bool,
}
