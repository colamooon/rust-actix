use std::time::SystemTime;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::post;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = post)]
pub struct Post {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub active: bool,
    pub created_at: Option<SystemTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<SystemTime>,
    pub updated_id: Option<i64>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostReq {
    pub title: String,
    pub body: String,
    pub created_at: Option<SystemTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<SystemTime>,
    pub updated_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRes {
    pub id: Option<i32>,
    pub active: bool,
    pub created_at: Option<SystemTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<SystemTime>,
    pub updated_id: Option<i64>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn render(post: &Post) -> PostRes {
    PostRes {
        id: post.id,
        active: post.active,
        created_at: post.created_at,
        created_id: post.created_id,
        updated_at: post.updated_at,
        updated_id: post.updated_id,
        title: post.title.to_owned(),
        body: post.body.to_owned(),
        published: post.published,
    }
}

fn naive_to_utc(val: Option<NaiveDateTime>) -> Option<DateTime<Utc>> {
    match val {
        None => None,
        Some(ref x) => Some(Utc.from_utc_datetime(x)), // x is now a string slice
    }
}
