DROP TABLE users;
DROP TABLE videos;

CREATE TABLE users(
    id SERIAL PRIMARY KEY NOT NULL,
    login TEXT NOT NULL,
    eth_addr TEXT NOT NULL,
    bio TEXT DEFAULT 'Hello, i am new on swell!',
    czar INTEGER DEFAULT 1000,
    videos INTEGER[] DEFAULT '{0}',
    videos_bought INTEGER[] DEFAULT '{0}',
    liked INTEGER[] DEFAULT '{0}'
);

CREATE TABLE videos(
    id SERIAL PRIMARY KEY NOT NULL,
    owner_id SERIAL NOT NULL,
    path TEXT NOT NULL,
    views INTEGER DEFAULT 0,
    liked INTEGER DEFAULT 0,
    price INTEGER DEFAULT 0
);