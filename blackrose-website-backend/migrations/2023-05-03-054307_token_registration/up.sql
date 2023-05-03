-- Your SQL goes here
ALTER TABLE users
ADD registration_confirmed bool NOT NULL;
CREATE TABLE registration_tokens (
    id int NOT NULL,
    registration_token text NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (id) REFERENCES users(id)
);