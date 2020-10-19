use std::convert::Infallible;
use warp::Reply;
use warp::reply::html;

pub(crate) mod math_handlers;

pub(crate) async fn generic_ok() -> Result<impl Reply, Infallible> {
    Ok(html(r#"<html>Hello!</html>"#))
}
