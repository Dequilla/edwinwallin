use std::fs;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

extern crate serde;
extern crate serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    address: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config_file = fs::read_to_string("/etc/edwinwallin/config.json").expect("Could not find config file in \"/etc/edwinwallin/config.json\"");
    let config: Config = serde_json::from_str(&config_file).expect("Config could not load. Failed to parse json to Config struct.");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.address, config.port))?
    .run()
    .await
}

