-- Your SQL goes here
CREATE TABLE member_info (
    id bigint auto_increment primary key,
    active boolean not null default true,
    created_at datetime(6) null,
    created_id bigint null,
    updated_at datetime(6) null,
    updated_id bigint null,
    username VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    password VARCHAR(255),
    sign_up_type VARCHAR(20) NOT NULL
)