#![deny(warnings)]
use hyper::server::Server;
use listenfd::ListenFd;
use std::convert::Infallible;
use warp::{Filter, Reply, Rejection};

pub async fn start_server<F>(routes: F)
where
    F: Filter<Error = Rejection> + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    // hyper let's us build a server from a TcpListener (which will be
    // useful shortly). Thus, we'll need to convert our `warp::Filter` into
    // a `hyper::service::MakeService` for use with a `hyper::server::Server`.
    let svc = warp::service(routes);

    let make_svc = hyper::service::make_service_fn(|_: _| {
        // the clone is there because not all warp filters impl Copy
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();
    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        let addr = env!("IP_ADDR").parse().unwrap();
        Server::bind(&addr)
    };

    server.serve(make_svc).await.unwrap();
}
