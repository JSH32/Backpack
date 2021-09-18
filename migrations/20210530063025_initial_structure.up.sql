-- Role enum
CREATE TYPE role AS ENUM ('user', 'admin');

-- Sonyflake type
CREATE DOMAIN sonyflake AS VARCHAR(20) NOT NULL;

-- Users table
CREATE TABLE users
(
    id       sonyflake  PRIMARY KEY        NOT NULL,
    email    VARCHAR(320)                  NOT NULL,
    username VARCHAR(32)                   NOT NULL,
    password VARCHAR(128)                  NOT NULL,
    verified BOOLEAN DEFAULT false         NOT NULL,
    role     role    DEFAULT 'user'::role  NOT NULL
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
    id          sonyflake  PRIMARY KEY  NOT NULL,
    user_id     sonyflake               NOT NULL,
    name        VARCHAR(32)             NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX tokens_id_uindex
    ON tokens (id);

CREATE TABLE verifications
(
	id          SERIAL  PRIMARY KEY  NOT NULL,
	user_id     sonyflake            NOT NULL,
	code        VARCHAR(72)          NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX verifications_code_uindex
	on verifications (code);

CREATE UNIQUE INDEX verifications_id_uindex
	on verifications (id);

CREATE TABLE files
(
    id         sonyflake  PRIMARY KEY  NOT NULL,
    owner_id   sonyflake               NOT NULL,
    name       VARCHAR(32)             NOT NULL,
    hash       VARCHAR(32)             NOT NULL,
    uploaded   timestamptz             NOT NULL,
    size       BIGINT                  NOT NULL,
    
    -- Application needs to delete the files from the S3 container. This is precautionary for database
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX files_id_uindex
	on files (id);

CREATE UNIQUE INDEX files_name_uindex
	on files (name);