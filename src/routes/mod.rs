use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error= Rejection> + Clone + Send + Sync + 'static {
    warp::path!("hello" / String).map(|name| format!("Hello {}!",name))
}
