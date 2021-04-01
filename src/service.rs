use std::convert::Infallible;
use std::env;

use hyper::{Body, Method, Request, Response, StatusCode};
use lazy_static::lazy_static;

lazy_static! {
    static ref WEBHOOK_PATH: String = env::var("WEBHOOK_PATH").unwrap_or("/default".into());
}

pub async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, path) if path == *WEBHOOK_PATH =>
            Ok(Response::new("Hello world".into())),

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
