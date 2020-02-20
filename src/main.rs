use std::sync::Arc;

use warp::Filter;
use refinery::config::Config;
use sqlx::postgres::PgPool;

mod util;
mod handlers;
mod routes;
mod models;
mod migrations;

#[tokio::main]
async fn main() {

    let mut refinery_config = Config::from_env_var("DATABASE_URL").unwrap();
    migrations::runner().run_async(&mut refinery_config).await.unwrap();
    let pool = PgPool::new(env!("DATABASE_URL")).await.unwrap();
    let pool = Arc::new(pool);

    util::tracing::start_tracing();
    let routes = routes::routes(pool).with(warp::trace::request());
    util::starter::start_server(routes).await;
}

