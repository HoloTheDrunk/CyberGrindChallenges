use serde_json::json;

use super::{Score, dto::CreateScore};

use crate::models::{
    model::Model,
    variations::Variation,
};

use {
    actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder},
    async_convert::TryInto as AsyncTryInto,
    serde::Deserialize,
};

#[get("/scores")]
async fn find_all() -> impl Responder {
    let scores = web::block(|| Score::find_all()).await.unwrap().await;
    match scores {
        Ok(scores) => HttpResponse::Ok().json(scores),
        Err(err) => HttpResponse::InternalServerError().json(format!("{err:?}")),
    }
}

#[get("/scores/{id}")]
async fn find(id: web::Path<i64>) -> impl Responder {
    let score = Score::find(id.into_inner()).await.unwrap();
    HttpResponse::Ok().json(score)
}

#[post("/scores")]
async fn create(score: web::Json<CreateScore>) -> impl Responder {
    let request = score.into_inner();

    let variation = match TryInto::<Variation>::try_into(request.variation) {
        Ok(variation) => variation,
        Err(_) => return HttpResponse::BadRequest().json(json!({ "error": "invalid weapons" })),
    };

    let variation_id = match Variation::match_weapons(&variation).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().json(json!({ "error": "unknown variation" })),
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({ "error": format!("{err:?}") }))
        }
    };

    if request.steam_id < 0 || request.score < 0 || request.progress < 0 {
        return HttpResponse::BadRequest()
            .json(json!({ "error": "steam_id, score and progress should be positive" }));
    }

    let score = Score {
        id: None,
        variation: variation_id,
        steam_id: request.steam_id,
        score: request.score,
        progress: request.progress,
    };

    let Ok(opt_score_id) = Score::insert(&score).await else { 
        return HttpResponse::InternalServerError().json(json!({ "error": "failed to insert score" }))
    };
    
    let Some(score_id) = opt_score_id else {
        return HttpResponse::Ok().json(json!({ "status": "not a high score" }))
    };

    HttpResponse::Ok().json(json!({ "status": "high score", "id": score_id }))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
}
