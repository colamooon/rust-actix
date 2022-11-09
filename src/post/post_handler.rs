use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;

use crate::{db_types::DbPool, models::PageInfo};

use super::{post::NewPostReq, post_service};

pub async fn get_posts(
    pool: web::Data<DbPool>,
    page_info: web::Query<PageInfo>,
) -> Result<HttpResponse, Error> {
    print!("]-----] post_handler::get_posts [------[");
    let posts = web::block(move || {
        let mut conn = pool.get()?;

        post_service::find_all_post(&mut conn, page_info.page, page_info.size)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(posts) = posts {
        Ok(HttpResponse::Ok().json(posts))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found"));
        Ok(res)
    }
}

pub async fn add_post(
    pool: web::Data<DbPool>,
    new_post_req: web::Json<NewPostReq>,
) -> Result<HttpResponse, Error> {
    print!("]-----] post_handler::add_post [------[");

    let posts = web::block(move || {
        let mut conn = pool.get()?;
        post_service::insert_new_post(&mut conn, &new_post_req)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}
