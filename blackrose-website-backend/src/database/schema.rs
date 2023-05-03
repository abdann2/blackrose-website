// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (blog_post_id) {
        blog_post_id -> Int4,
        title -> Text,
        content -> Text,
        author_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        removed -> Bool,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Int4,
        content -> Text,
        author_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        removed -> Bool,
    }
}

diesel::table! {
    registration_tokens (user_id) {
        user_id -> Int4,
        registration_token -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Text,
        display_name -> Text,
        email -> Text,
        password_hash -> Text,
        admin -> Bool,
        registration_confirmed -> Bool,
    }
}

diesel::joinable!(blog_posts -> users (author_id));
diesel::joinable!(comments -> blog_posts (post_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(registration_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_posts,
    comments,
    registration_tokens,
    users,
);
