use actix_web::{web, App, HttpServer};
use std::env;
mod mongo;
use mongodb::{options::ClientOptions, Client};
use std::sync::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    let mut client_options = ClientOptions::parse("mongodb://mongosopes1p1:ij8uDqTbzpPyFot5sB7ozhiOpOw5dZZo2sV3fAscHvLHkBAvRgK9YpBrykNSijGbFATiGIJzDrgfVaKnliOZMQ==@mongosopes1p1.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@mongosopes1p1@").await.unwrap();
    client_options.app_name = Some("mongosopes1p1".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .route("/", web::get().to(|| async{"Api en Rust...⌚"}))
            .service(web::scope("/carga").configure(mongo::scoped_config))
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}