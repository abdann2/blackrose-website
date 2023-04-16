// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Int4,
        title -> Nullable<Text>,
        content -> Nullable<Text>,
        author_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        removed -> Nullable<Bool>,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Nullable<Text>,
        author_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        removed -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Text>,
        display_name -> Nullable<Text>,
        email -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        admin -> Nullable<Bool>,
    }
}

diesel::joinable!(blog_posts -> users (author_id));
diesel::joinable!(comments -> blog_posts (post_id));
diesel::joinable!(comments -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_posts,
    comments,
    users,
);
