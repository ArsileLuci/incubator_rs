-- Your SQL goes here
CREATE TABLE users (
    id          SERIAL      NOT NULL PRIMARY KEY,
    name        varchar     NOT NULL UNIQUE,
    password    varchar     NOT NULL
);

CREATE TABLE friends (
    user_id     integer     NOT NULL REFERENCES users (id),
    friend_id   integer     NOT NULL REFERENCES users (id),
    CONSTRAINT  user_friend PRIMARY KEY (user_id, friend_id)
);
