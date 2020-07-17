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