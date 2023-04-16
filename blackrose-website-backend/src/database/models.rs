use crate::database::schema::{blog_posts, comments, users};
use diesel::data_types::PgTimestamp;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = blog_posts)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    author_id: i32,
    created_at: PgTimestamp,
    updated_at: PgTimestamp,
    removed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = blog_posts)]
struct NewBlogPost {
    title: String,
    content: String,
    author_id: i32,
    created_at: PgTimestamp,
    updated_at: PgTimestamp,
    removed: bool,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    username: String,
    display_name: String,
    email: String,
    password_hash: String,
    admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    display_name: String,
    email: String,
    password_hash: String,
    admin: bool,
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
