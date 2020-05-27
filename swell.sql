CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY NOT NULL,
    login TEXT NOT NULL,
    eth_addr TEXT NOT NULL,
    bio TEXT DEFAULT 'Hello, i am new on swell!',
    czar INTEGER DEFAULT 1000,
    followers INTEGER DEFAULT 0,
);