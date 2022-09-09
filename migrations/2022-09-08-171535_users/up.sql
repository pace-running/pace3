-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

INSERT INTO users(username,password_hash,role) VALUES('admin','$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$OOA07UjKrh3ijWboNB5/Ur274nxXirUuifmSuGCXwY0','admin');