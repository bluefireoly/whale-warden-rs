use std::env;

use actix_web::{App, HttpServer, web};

use service::hello_world;

mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("WEBHOOK_PORT").unwrap_or("9090".into());
    let route = env::var("WEBHOOK_PATH").unwrap_or("/webhook/update".into());

    HttpServer::new(move || {
        App::new()
            .route(&route, web::get().to(hello_world))
    })
        .bind("127.0.0.1:".to_owned() + &port)?
        .run()
        .await
}
