use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Serialize, Deserialize, Debug)]
pub enum Rarity {
    COMMON,
    UNUSUAL,
    RARE,
    VERY_RARE,
    LEGENDARY,
    ARTIFACT,
    VARIED,
}