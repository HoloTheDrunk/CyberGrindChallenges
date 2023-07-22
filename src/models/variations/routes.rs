use super::Variation;

use crate::models::model::Model;

use {
    actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder},
    serde::Deserialize,
    serde_json::json,
};

#[derive(Deserialize)]
pub struct Weapons {
    pub weapons: Vec<String>,
}

impl TryInto<Variation> for Weapons {
    type Error = ();

    fn try_into(self) -> Result<Variation, Self::Error> {
        let mut variation = Variation::default();
        if self.weapons.iter().all(|weapon| variation.try_use(weapon)) {
            Ok(variation)
        } else {
            Err(())
        }
    }
}

#[post("/variations")]
async fn create(allowed: web::Json<Weapons>) -> impl Responder {
    match allowed.into_inner().try_into() {
        Ok(variation) => {
            let insert_result = Variation::insert(&variation).await;
            match insert_result {
                Ok(Some(id)) => HttpResponse::Ok().json(json!({ "id": id })),
                Ok(None) => {
                    HttpResponse::Conflict().json(json!({ "error": "Variation already exists" }))
                }
                Err(err) => {
                    HttpResponse::InternalServerError().json(json!({ "error": format!("{err:?}") }))
                }
            }
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({ "error": "One of the weapons is invalid" }))
        }
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}
