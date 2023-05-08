use crate::database::schema::{blog_posts, comments, registration_tokens, users};
use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = blog_posts)]
#[diesel(primary_key(blog_post_id))]
pub struct BlogPost {
    #[diesel(column_name = blog_post_id)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub removed: bool,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = blog_posts)]
pub struct NewBlogPost {
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub removed: bool,
}

#[derive(Queryable, AsChangeset, Clone, Identifiable, Selectable)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    #[diesel(column_name = user_id)]
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub admin: bool,
    pub registration_confirmed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub admin: bool,
    pub registration_confirmed: bool,
}

#[derive(Queryable, Identifiable, Insertable, Associations, Selectable)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User))]
pub struct RegistrationToken {
    pub user_id: i32,
    pub registration_token: String,
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

#[derive(Deserialize)]
pub struct RegistrationQueryExtractor {
    pub registration_token: String,
}
#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(BlogPost, foreign_key = post_id))]
#[diesel(primary_key(comment_id))]
#[diesel(table_name = comments)]
struct Comment {
    #[diesel(column_name = comment_id)]
    id: i32,
    content: String,
    author_id: i32,
    post_id: i32,
    created_at: PgTimestamp,
    updated_at: Option<PgTimestamp>,
    removed: bool,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(BlogPost, foreign_key = post_id))]
#[diesel(table_name = comments)]
struct NewComment {
    content: String,
    author_id: i32,
    post_id: i32,
    created_at: PgTimestamp,
    updated_at: Option<PgTimestamp>,
}
