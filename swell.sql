DROP TABLE messages cascade;
--DROP TABLE thread_participant cascade ;
--DROP TABLE threads cascade;
DROP TABLE products cascade;
DROP TABLE medias cascade;
DROP TABLE follows cascade;
DROP TABLE users cascade;
DROP TYPE PRODUCT_TYPE;
DROP TYPE MEDIA_TYPE;

SET timezone TO 'UTC';
ALTER DATABASE swell SET timezone TO 'Europe/Paris';

CREATE TYPE PRODUCT_TYPE as ENUM('MEDIA', 'REAL');
CREATE TYPE MEDIA_TYPE as ENUM('VIDEO', 'IMAGE');

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    eth_address TEXT NOT NULL UNIQUE,
    bio TEXT DEFAULT 'Hello, i am new on Squarrin',
    quadreum BIGINT DEFAULT 1000,
    avatar TEXT DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TABLE follows(
    followee_id BIGINT NOT NULL REFERENCES users(id),
    follower_id BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (followee_id, follower_id),
    PRIMARY KEY (followee_id, follower_id)
);

CREATE TABLE medias (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    path TEXT NOT NULL UNIQUE,
    thumbnail_path TEXT DEFAULT '',
    media_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    product_type TEXT NOT NULL,
    seller_id BIGINT NOT NULL REFERENCES users(id),
    buyers_id BIGINT DEFAULT 0,
    description TEXT DEFAULT '' NOT NULL,
    price BIGINT DEFAULT 0,
    media_id BIGINT NOT NULL REFERENCES medias(id),
    views BIGINT DEFAULT 0,
    likes BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/*
CREATE TABLE threads (
    id BIGSERIAL PRIMARY KEY NOT NULL
);

CREATE TABLE thread_participant(
    thread_id BIGINT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES threads(id) ON DELETE CASCADE
);
*/
/*
CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    sender_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);*/


CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    sender BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
