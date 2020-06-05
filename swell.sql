DROP TABLE users;
DROP TABLE videos;

CREATE TABLE users(
    id SERIAL PRIMARY KEY NOT NULL,
    login TEXT NOT NULL UNIQUE,
    eth_addr TEXT NOT NULL UNIQUE,
    bio TEXT DEFAULT 'Hello, i am new on swell!',
    czar INTEGER DEFAULT 1000,
    videos INTEGER[] DEFAULT '{}',
    videos_bought INTEGER[] DEFAULT '{}',
    liked INTEGER[] DEFAULT '{}'
);

CREATE TABLE videos(
    id SERIAL PRIMARY KEY NOT NULL,
    owner_id SERIAL NOT NULL,
    title TEXT,
    bio TEXT,
    price INTEGER DEFAULT 0,
    path TEXT NOT NULL,
    views INTEGER DEFAULT 0,
    liked INTEGER DEFAULT 0
);