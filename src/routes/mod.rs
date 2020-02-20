use std::convert::Infallible;
use std::sync::Arc;

use tera::Context;
use tera::Tera;
use crate::TEMPLATES;
use warp::{Filter, Rejection, Reply, reply::html};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error= Rejection> + Clone + Send + Sync + 'static {
    warp::path::end().and(with_templates()).map(|templates:Arc<Tera>| {
        let mut context = Context::new();
        context.insert("number", &2);
        html(templates.render("index.html", &context).unwrap())
    })
}

pub fn with_templates() -> impl Filter<Extract = (Arc<Tera>,), Error = Infallible>  + Clone + Send + Sync + 'static {
    warp::any().map(move || TEMPLATES.clone())
}
