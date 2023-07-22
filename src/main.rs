#![allow(unused)]

mod db;
mod models;

use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

use models::{scores::Score, variations::Variation};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "online"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    pretty_env_logger::init();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .service(status)
            .configure(models::scores::init_routes)
            .configure(models::variations::init_routes)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
