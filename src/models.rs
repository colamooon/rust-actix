use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct MemberInfo {
    pub id: i32,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub created_id: Option<i64>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_id: Option<i64>,
    pub username: String,
    pub display_name: String,
    pub password: Option<String>,
    pub sign_up_type: String,
}


#[derive(Deserialize)]
pub struct PageInfo {
    pub page: i64,
    pub size: i64,
}
