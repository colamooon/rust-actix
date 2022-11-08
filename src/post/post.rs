use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::post;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = post)]
pub struct Post {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_id: Option<i64>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostReq {
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRes {
    pub id: Option<i64>,
    pub active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub created_id: Option<i64>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_id: Option<i64>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn render(post: &Post) -> PostRes {
    PostRes {
        id: post.id,
        active: post.active,
        created_at: naive_to_utc(post.created_at),
        created_id: post.created_id,
        updated_at: naive_to_utc(post.updated_at),
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
