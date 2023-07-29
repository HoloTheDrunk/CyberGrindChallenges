use serde_json::json;

use {actix_web::HttpResponse, thiserror::Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Provided weapons list is invalid")]
    InvalidWeapons,
}

impl From<dotenv::Error> for Error {
    fn from(value: dotenv::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

impl Into<HttpResponse> for Error {
    fn into(self) -> HttpResponse {
        match self {
            Error::Internal(err) => {
                HttpResponse::InternalServerError().json(json!({ "error": err }))
            }
            err @ Error::Unauthorized => {
                HttpResponse::Unauthorized().json(json!({ "error": err.to_string() }))
            }
            err @ Error::InvalidWeapons => {
                HttpResponse::BadRequest().json(json!({ "error": err.to_string() }))
            }
        }
    }
}
