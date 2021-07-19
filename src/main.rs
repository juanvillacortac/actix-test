use mongodb::{options::ClientOptions, Client};
use std::{env, io};

mod constants;
mod handlers;

use handlers::test;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("smartdat");

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(test::get_test)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
