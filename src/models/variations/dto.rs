use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weapons {
    pub weapons: Vec<String>,
}
