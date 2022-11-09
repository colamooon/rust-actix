-- Your SQL goes here
CREATE TABLE member_info (
    id SERIAL primary key,
    active boolean not null default true,
    created_at timestamp null,
    created_id bigint null,
    updated_at timestamp null,
    updated_id bigint null,
    username VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    password VARCHAR(255),
    sign_up_type VARCHAR(20) NOT NULL
)