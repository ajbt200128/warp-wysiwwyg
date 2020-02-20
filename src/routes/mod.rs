use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::handlers;
pub fn routes(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + Send + Sync + 'static {
    warp::path!("hello" / String)
        .and(with_db(db))
        .and_then(handlers::db_get_user_handler)
}

fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone + Send + Sync + 'static
{
    warp::any().map(move || db.clone())
}
