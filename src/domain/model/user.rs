use crate::domain::model::UserId;
use diesel::sql_types::Timestamptz;
use diesel::{Insertable, Queryable};
use twitter2_api::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
