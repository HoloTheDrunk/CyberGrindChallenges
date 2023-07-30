use super::{dto::Weapons, Variation};

use crate::models::{
    error::Error,
    model::{Model, Protected},
};

use {
    actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder},
    serde::Deserialize,
    serde_json::json,
};

impl TryInto<Variation> for Weapons {
    type Error = Error;

    fn try_into(self) -> Result<Variation, Self::Error> {
        let mut variation = Variation::default();
        self.weapons
            .iter()
            .all(|weapon| variation.try_use(weapon))
            .then_some(variation)
            .ok_or(Error::InvalidWeapons)
    }
}

#[post("/variations")]
async fn create(request: web::Json<Protected<Weapons>>) -> HttpResponse {
    let weapons = match request.into_inner().validate() {
        Ok(variations) => variations,
        Err(error) => return error.into(),
    };

    match weapons.try_into() {
        Ok(variation) => {
            let insert_result = Variation::insert(&variation).await;
            match insert_result {
                Ok(Some(id)) => HttpResponse::Ok().json(json!({ "id": id })),
                Ok(None) => Error::Conflict.into(),
                Err(err) => Error::Internal(err.to_string()).into(),
            }
        }
        Err(err) => err.into(),
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}
