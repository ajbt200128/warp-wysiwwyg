use warp::Filter;
use warp::Rejection;
use warp::Reply;

mod util;

mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    util::tracing::start_tracing();
    let routes = routes::routes().with(warp::trace::request());
    util::starter::start_server(routes).await;
}

