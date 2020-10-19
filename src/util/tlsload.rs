use warp::{Filter, Reply, Rejection};
use std::net::SocketAddr;

pub async fn start_server<F>(routes: F)
where
    F: Filter<Error = Rejection> + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    let addr: SocketAddr = env!("IP_ADDR").parse().unwrap();
    warp::serve(routes)
        .tls()
        .cert_path(env!("TLS_CERT"))
        .key_path(env!("TLS_KEY"))
        .run(addr)
        .await;
}
