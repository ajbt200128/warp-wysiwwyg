use crate::handlers::math_handlers;
use warp::Filter;
use warp::Rejection;
use warp::Reply;


pub fn math(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + Send + Sync + 'static {
    let help = warp::path!("help" / String).and_then(math_handlers::help_with_func);
    let math = warp::path("math").and(sum().or(times()).or(help));
    let base = warp::path("math")
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16 or /math/help/:str");
    base.or(math)
}

fn sum() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static {
    warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b))
}

fn times() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static {
    warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b))
}
