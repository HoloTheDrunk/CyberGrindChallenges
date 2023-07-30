use serde_json::json;

use {actix_web::HttpResponse, thiserror::Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ItemType {
    Score,
    Variation,
    User,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Provided weapons list is invalid")]
    InvalidWeapons,
    #[error("{0:?} not found")]
    NotFound(ItemType),
    #[error("Item already exists")]
    Conflict,
    #[error("Steam ID, score and progress must be positive")]
    UnvalidatedConstraints,
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
            Error::Unauthorized => {
                HttpResponse::Unauthorized().json(json!({ "error": self.to_string() }))
            }
            Error::InvalidWeapons => {
                HttpResponse::BadRequest().json(json!({ "error": self.to_string() }))
            }
            Error::NotFound(_) => {
                HttpResponse::NotFound().json(json!({ "error": self.to_string() }))
            }
            Error::Conflict => HttpResponse::Conflict().json(json!({ "error": self.to_string() })),
            Error::UnvalidatedConstraints => {
                HttpResponse::BadRequest().json(json!({ "error": self.to_string() }))
            }
        }
    }
}
