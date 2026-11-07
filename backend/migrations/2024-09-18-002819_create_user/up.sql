-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR (36) PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password TEXT NOT NULL
)
