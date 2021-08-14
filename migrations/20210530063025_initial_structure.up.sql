-- Role enum
CREATE TYPE Role AS ENUM ('User', 'Admin');

-- Users table
CREATE TABLE users
(
    id       SERIAL                        NOT NULL,
    email    VARCHAR(320)                  NOT NULL,
    username VARCHAR(32)                   NOT NULL,
    password VARCHAR(128)                  NOT NULL,
    verified BOOLEAN DEFAULT false         NOT NULL,
    role     Role    DEFAULT 'User'::Role  NOT NULL
);

CREATE UNIQUE INDEX users_email_uindex
    ON users (email);

CREATE UNIQUE INDEX users_id_uindex
    ON users (id);

CREATE UNIQUE INDEX users_username_uindex
    ON users (username);

-- API token table for applications
CREATE TABLE tokens
(
    id          SERIAL       NOT NULL,
    user_id     INTEGER      NOT NULL,
    name        VARCHAR(32)  NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX tokens_id_uindex
    ON tokens (id);

CREATE TABLE verifications
(
	id          SERIAL       NOT NULL,
	user_id     INTEGER      NOT NULL,
	code        VARCHAR(72)  NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX verifications_code_uindex
	on verifications (code);

CREATE UNIQUE INDEX verifications_id_uindex
	on verifications (id);