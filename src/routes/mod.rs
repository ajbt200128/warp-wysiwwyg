use warp::{Filter, Rejection, Reply};
use crate::handlers;

mod math;

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error= Rejection> + Clone + Send + Sync + 'static {
    let hello = warp::path!("hello").and_then(handlers::generic_ok);
    warp::get()
        .and(math::math().or(hello))
}
