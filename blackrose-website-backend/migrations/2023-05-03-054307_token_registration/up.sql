-- Your SQL goes here
ALTER TABLE users
ADD registration_confirmed bool NOT NULL;
CREATE TABLE registration_tokens (
    user_id int NOT NULL,
    registration_token text NOT NULL,
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);