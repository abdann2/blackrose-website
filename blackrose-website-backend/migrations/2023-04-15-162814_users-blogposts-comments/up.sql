-- Your SQL goes here
CREATE TABLE users (
    user_id serial PRIMARY KEY,
    username text NOT NULL,
    display_name text NOT NULL,
    email text NOT NULL,
    password_hash text NOT NULL,
    admin bool NOT NULL
);
CREATE TABLE blog_posts (
    blog_post_id serial primary key NOT NULL,
    title text NOT NULL,
    content text NOT NULL,
    author_id int NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz,
    removed bool NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (user_id)
);
CREATE TABLE comments (
    comment_id serial PRIMARY KEY NOT NULL,
    content text NOT NULL,
    author_id int NOT NULL,
    post_id int NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz,
    removed bool NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (user_id),
    FOREIGN KEY (post_id) REFERENCES blog_posts (blog_post_id)
);