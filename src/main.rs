#![allow(unused)]

mod db;
mod models;

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     pretty_env_logger::init();
//
//     HttpServer::new(move || {
//         let cors = actix_cors::Cors::permissive();
//
//         App::new().wrap(cors)
//     })
//     .bind(("localhost", 8080))?
//     .run()
//     .await
// }

use models::{scores::Score, variations::Variation};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let mut variation = Variation::default();
    variation.use_piercer();

    dbg!(variation);
}
