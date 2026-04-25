use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    COMMON,
    UNUSUAL,
    RARE,
    VERY_RARE,
    LEGENDARY,
    ARTIFACT,
    VARIED,
}