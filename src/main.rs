use std::sync::Arc;

use warp::Filter;

use lazy_static::lazy_static;

use tera::Tera;

mod util;

mod handlers;
mod routes;
mod models;

lazy_static! {
    pub(crate) static ref TEMPLATES: Arc<Tera> = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        if cfg!(feature = "autoreload"){
            tera.full_reload().unwrap();
        }
        Arc::new(tera)
    };
}

#[tokio::main]
async fn main() {

    util::tracing::start_tracing();
    let routes = routes::routes().with(warp::trace::request());
    util::starter::start_server(routes).await;
}

