use std::time::SystemTime;

use crate::schema::posts;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub created_at: SystemTime,
    pub author: String,
}

#[derive(Serialize)]
pub struct GetPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub created_at: String,
    pub author: String,
}

impl From<&Post> for GetPost {
    fn from(value: &Post) -> Self {
        let date: DateTime<Utc> = value.created_at.into();
        GetPost {
            id: value.id,
            title: value.title.clone(),
            body: value.body.clone(),
            is_public: value.is_public,
            slug: value.slug.clone(),
            created_at: format!("{}", date.format("%+")),
            author: value.author.clone(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub is_public: bool,
    pub slug: String,
    pub author: String,
}

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub iat: usize,
}
