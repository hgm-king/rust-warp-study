use crate::{error_reply, models, server::Context, views, NotFound};
use hyper::StatusCode;
use std::convert::Infallible;
use warp::{reject, Rejection, Reply};

pub async fn profile(
    context: Context,
    user: models::user::User,
    _session: models::session::Session,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let pages = models::page::read_pages_by_user_id(&mut conn, user.id)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .map(|page| views::page::list_item(page))
        .collect::<String>();
    let profile_html = views::user::profile(user).replace("{pages}", &pages);

    Ok(warp::reply::html(profile_html))
}

pub async fn profile_with_cookie(
    context: Context,
    user: models::user::User,
    session: models::session::Session,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let pages = models::page::read_pages_by_user_id(&mut conn, user.id)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .map(|page| views::page::list_item(page))
        .collect::<String>();
    let profile_html = views::user::profile(user).replace("{pages}", &pages);

    Ok(warp::reply::with_header(
        warp::reply::html(profile_html),
        "Set-Cookie",
        format!("session={}; path=/", session.id),
    ))
}

pub async fn logout() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_header(
        warp::reply::html(views::body::index("You have logged out")),
        "Set-Cookie",
        format!("session=; Path=/"),
    ))
}

pub async fn signup_form() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::user::signup_form()))
}

pub async fn login_form() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::user::login_form("")))
}

pub async fn handle_login_errors(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(NotFound) = err.find::<NotFound>() {
        let html = views::user::login_form("Error: Invalid login credentials");
        error_reply(StatusCode::NOT_FOUND, html)
    } else {
        Err(err)
    }
}

pub async fn handle_logout_errors(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_) = err.find::<reject::MissingCookie>() {
        let html = views::body::index("Error: Not logged in to begin with");
        error_reply(StatusCode::BAD_REQUEST, html)
    } else {
        Err(err)
    }
}
