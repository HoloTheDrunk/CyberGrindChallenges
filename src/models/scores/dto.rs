use crate::models::variations::dto::Weapons;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateScore {
    pub variation: Weapons,
    pub steam_id: i64,
    pub score: i32,
    pub progress: i32,
}
