use crate::{models, server::Context};
use std::sync::Arc;
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("page").boxed()
}

pub fn get_by_id() -> BoxedFilter<(Context, models::page::Page)> {
    warp::get()
        .and(path_prefix())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

async fn with_page(
    context: Context,
    id: i32,
) -> Result<(Context, models::page::Page), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id(&mut conn, id).map_err(|_| reject::not_found())?;
    Ok((context, page))
}

pub fn create() -> BoxedFilter<(Context, models::page::Page)> {
    warp::post()
        .and(path_prefix())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::page::NewPageApi>())
        .and_then(insert_new_page)
        .untuple_one()
        .boxed()
}

async fn insert_new_page(
    context: Context,
    new_page: models::page::NewPageApi,
) -> Result<(Context, models::page::Page), warp::Rejection> {
    log::info!("Saving Page");
    let mut conn = context.db_conn.get_conn();
    let page = models::page::NewPage::new(new_page)
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            reject::reject()
        })?;
    log::info!("Saved Page");
    Ok((context, page))
}

pub fn create_form() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("create"))
        .boxed()
}

pub fn create_link() -> BoxedFilter<(Context, models::page::Page)> {
    warp::post()
        .and(path_prefix())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(warp::body::form::<models::link::NewLinkApi>())
        .and_then(with_new_link)
        .untuple_one()
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

async fn with_new_link(
    context: Context,
    page_id: i32,
    new_link: models::link::NewLinkApi,
) -> Result<(Context, i32), warp::Rejection> {
    log::info!("Saving Link");
    let mut conn = context.db_conn.get_conn();

    let link = match models::link::read_by_url(&mut conn, new_link.url.clone()) {
        Err(diesel::NotFound) => models::link::NewLink::new(new_link)
            .insert(&mut conn)
            .map_err(|_| reject::reject()),
        Ok(link) => Ok(link),
        _ => Err(warp::reject()),
    }?;

    models::page_link::NewPageLink::new(page_id, link.id)
        .insert(&mut conn)
        .map_err(|_| reject::reject())?;

    log::info!("Saved Link");
    Ok((context, page_id))
}
