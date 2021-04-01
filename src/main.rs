use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};

use service::hello_world;

mod service;

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
