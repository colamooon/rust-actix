-- Your SQL goes here
CREATE TABLE post (
    id SERIAL primary key,
    active boolean not null default true,
    created_at timestamp null,
    created_id bigint null,
    updated_at timestamp null,
    updated_id bigint null,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published boolean not null default false
)