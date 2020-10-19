use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error= Rejection> + Clone + Send + Sync + 'static {
}
