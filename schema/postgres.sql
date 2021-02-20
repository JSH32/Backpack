-- User role enum
DO
$do$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_type
      WHERE typname = 'role') THEN

      CREATE TYPE role AS ENUM ('user', 'admin');
   END IF;
END
$do$;

-- Users table
CREATE TABLE IF NOT EXISTS users
(
    id       SERIAL                NOT NULL,
    email    VARCHAR(320)          NOT NULL,
    username VARCHAR(32)           NOT NULL,
    password VARCHAR(128)          NOT NULL,
    verified boolean DEFAULT false NOT NULL
    role     role    DEFAULT 'user'::role
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_uindex
    ON users (email);

CREATE UNIQUE INDEX IF NOT EXISTS users_id_uindex
    ON users (id);

CREATE UNIQUE INDEX IF NOT EXISTS users_username_uindex
    ON users (username);