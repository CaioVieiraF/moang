use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
