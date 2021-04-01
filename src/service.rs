use std::env;

use actix_web::{HttpResponse, Responder};
use actix_web::web::Query;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref WEBHOOK_TOKEN: String = env::var("WEBHOOK_TOKEN").unwrap_or("unset".into());
}

#[derive(Deserialize)]
pub struct Webhook {
    token: String,
}

pub async fn hello_world(info: Query<Webhook>) -> impl Responder {
    if info.token == *WEBHOOK_TOKEN {
        // docker logic
        return HttpResponse::Ok().body("Webhook succeeded")
    }
    return HttpResponse::Forbidden().body("Wrong token")
}
