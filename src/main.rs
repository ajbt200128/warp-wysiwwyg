use warp::Filter;
use warp::Rejection;
use warp::Reply;

mod util;

mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    // Wrap all the routes with a filter that creates a `tracing` span for
    // each request we receive, including data about the request.
    util::tracing::start_tracing();
    let routes = routes::routes().with(warp::trace::request());
    util::starter::start_server(routes).await;
}

