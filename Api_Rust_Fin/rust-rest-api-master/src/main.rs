use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer, web};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use user_service::UserService;

mod user_router;
mod user_service;

pub struct ServiceManager {
    user: UserService,
}

impl ServiceManager {
    pub fn new(user: UserService) -> Self {
        ServiceManager { user }
    }
}

pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init env
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Parse a connection string into an options struct.
    let database_url = "mongodb://mongosopes1p1:ij8uDqTbzpPyFot5sB7ozhiOpOw5dZZo2sV3fAscHvLHkBAvRgK9YpBrykNSijGbFATiGIJzDrgfVaKnliOZMQ==@mongosopes1p1.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@mongosopes1p1@";
    let client_options = ClientOptions::parse(&database_url).unwrap();

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // Get a handle to a database.
    let database_name = "local";
    let db = client.database(&database_name);

    // Get a handle to a collection in the database.
    let user_collection_name ="tweet";
    let user_collection = db.collection(&user_collection_name);

    // server url
    let server_url = "0.0.0.0:3030";

    // start server
    HttpServer::new(move || {
        let user_service_worker = UserService::new(user_collection.clone());
        let service_manager = ServiceManager::new(user_service_worker);

        // cors
        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        // launch http server
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(user_router::init)
            .route("/", web::get().to(|| async{"Api en Rust...⌚"}))
    })
    .bind(server_url)?
    .run()
    .await
}
