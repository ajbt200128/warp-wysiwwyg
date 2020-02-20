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

    let mut refinery_config = Config::from_env_var("DB_URL").unwrap();
    migrations::runner().run_async(&mut refinery_config).await.unwrap();
    let pool = PgPool::new(env!("DB_URL")).await.unwrap();

    util::tracing::start_tracing();
    let routes = routes::routes().with(warp::trace::request());
    util::starter::start_server(routes).await;
}

