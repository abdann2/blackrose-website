use crate::database::schema::{blog_posts, comments, users};
use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = blog_posts)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub removed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = blog_posts)]
pub struct NewBlogPost {
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub removed: bool,
}

#[derive(Queryable, AsChangeset, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub admin: bool,
}

#[derive(Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRegistrationCredentials {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
#[diesel(table_name = comments)]
struct Comment {
    id: i32,
    content: String,
    author_id: i32,
    post_id: i32,
    created_at: PgTimestamp,
    updated_at: PgTimestamp,
    removed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
struct NewComment {
    content: String,
    author_id: i32,
    post_id: i32,
    created_at: PgTimestamp,
    updated_at: PgTimestamp,
}
