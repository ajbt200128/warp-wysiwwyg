use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::html;
use warp::reply::with_status;
use warp::Reply;
pub(crate) async fn help_with_func(name: String) -> Result<impl warp::Reply, Infallible> {
    match name.as_ref() {
        "sum" => Ok(with_status(
            html(r#"<html>GET /math/sum/:u32/:u32 <br> Sums two 32 bit ints</html>"#),
            StatusCode::OK,
        )),
        "times" => Ok(with_status(
            html("<html>GET /math/:u16/times/:u16 multiplies 2 16 bit ints</html>"),
            StatusCode::OK,
        )),
        _ => Ok(with_status(
            html(r#"<html>Function not found!</html>"#),
            StatusCode::NOT_FOUND,
        )),
    }
}

