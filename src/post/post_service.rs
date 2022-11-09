use crate::db_types::DbError;
use crate::pagination::*;
use crate::post::post::*;
use chrono::Utc;
use diesel::{insert_into, prelude::*, result::Error};
use log::info;

use super::post::NewPostReq;

pub fn find_all_post(
    conn: &mut MysqlConnection,
    page: i64,
    size: i64,
) -> Result<Option<Vec<PostRes>>, DbError> {
    info!("]-----] post_service::find_all_post [------[ {}", page);
    use crate::schema::post::dsl::*;

    // let finded_posts: Option<Vec<Post>> = post.load::<Post>(conn).optional()?;
    let per_page: Option<i64> = Some(size.to_owned());
    let page_index: i64 = if page > 0 { page } else { 1i64 };
    let page_index = page;

    let mut query = post
        .order(created_at.desc())
        .filter(active.eq(true))
        .paginate(page_index);
    info!(
        "]-----] post_service::find_all_post.query [------[ {:?}",
        query
    );

    if let Some(per_page) = per_page {
        use std::cmp::min;
        query = query.per_page(min(per_page, 25));
    }
    // info!("]-----] post_service::find_all_post.query [------[ {:?}", query);

    let (posts, total_pages) = query.load_and_count_pages::<Post>(conn)?;

    let to_display = posts.into_iter();
    let mut post_res: Vec<PostRes> = Vec::new();
    for post_in in to_display {
        post_res.push(crate::post::post::render(&post_in));
    }
    println!("Page {} of {}", page_index, total_pages);
    Ok(Some(post_res))
}

pub fn insert_new_post(
    conn: &mut MysqlConnection,
    new_post_req: &NewPostReq,
) -> Result<Post, DbError> {
    print!(
        "]-----] post_service::insert_new_post.new_post_req [------[ {:?}",
        new_post_req
    );
    use crate::schema::post::dsl::*;

    let inserted_post = conn.transaction::<_, Error, _>(|conn| {
        // let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
        let now = Utc::now().naive_utc();
        insert_into(post)
            .values((
                title.eq(new_post_req.title.to_owned()),
                body.eq(new_post_req.body.to_owned()),
                created_at.eq(now),
                updated_at.eq(now),
            ))
            .execute(conn)?;
        post.order(id.desc()).first(conn)
    })?;
    Ok(inserted_post)
}
