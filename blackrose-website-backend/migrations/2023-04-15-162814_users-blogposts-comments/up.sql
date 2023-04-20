-- Your SQL goes here
CREATE TABLE users (
    id serial PRIMARY KEY,
    username text NOT NULL,
    display_name text NOT NULL,
    email text NOT NULL,
    password_hash text NOT NULL,
    admin bool NOT NULL
);
CREATE TABLE blog_posts (
    id serial primary key NOT NULL,
    title text NOT NULL,
    content text NOT NULL,
    author_id int NOT NULL,
    created_at timestamp NOT NULL,
    updated_at timestamp NOT NULL,
    removed bool NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (id)
);
CREATE TABLE comments (
    id serial PRIMARY KEY NOT NULL,
    content text NOT NULL,
    author_id int NOT NULL,
    post_id int NOT NULL,
    created_at timestamp NOT NULL,
    updated_at timestamp NOT NULL,
    removed bool NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (id),
    FOREIGN KEY (post_id) REFERENCES blog_posts (id)
);