CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY NOT NULL,
    eth_addr TEXT NOT NULL,
    bio TEXT DEFAULT 'Hello, i am new on swell!'
);