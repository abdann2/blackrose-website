-- Your SQL goes here
CREATE TABLE users (
    id serial PRIMARY KEY,
    username text,
    display_name text,
    email text,
    password_hash text,
    admin bool
);
CREATE TABLE blog_posts (
    id serial primary key,
    title text,
    content text,
    author_id int,
    created_at timestamp,
    updated_at timestamp,
    removed bool,
    FOREIGN KEY (author_id) REFERENCES users (id)
);
CREATE TABLE comments (
    id serial PRIMARY KEY,
    content text,
    author_id int,
    post_id int,
    created_at timestamp,
    updated_at timestamp,
    removed bool,
    FOREIGN KEY (author_id) REFERENCES users (id),
    FOREIGN KEY (post_id) REFERENCES blog_posts (id)
);