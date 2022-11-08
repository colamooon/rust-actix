-- Your SQL goes here
CREATE TABLE post (
    id bigint auto_increment primary key,
    active boolean not null default true,
    created_at datetime(6) null,
    created_id bigint null,
    updated_at datetime(6) null,
    updated_id bigint null,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published boolean not null default false
)