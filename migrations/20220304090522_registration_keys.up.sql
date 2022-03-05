CREATE TABLE registration_keys
(
    id          sonyflake  PRIMARY KEY NOT NULL,
    code        uuid DEFAULT gen_random_uuid() NOT NULL UNIQUE,
    expiry_date timestamptz,
    iss_user     sonyflake  NOT NULL,
    used        INTEGER DEFAULT 0 NOT NULL,
    max_uses    INTEGER DEFAULT 1 NOT NULL,
    FOREIGN KEY (iss_user) REFERENCES users (id) ON DELETE CASCADE
);