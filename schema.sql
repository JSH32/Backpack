-- Users table
create table users
(
    id       serial                not null,
    email    varchar(320)          not null,
    username varchar(32)           not null,
    password varchar(128),
    verified boolean default false not null
);

alter table users
    owner to postgres;

create unique index users_email_uindex
    on users (email);

create unique index users_id_uindex
    on users (id);

create unique index users_username_uindex
    on users (username);