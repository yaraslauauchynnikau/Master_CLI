use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuffType {
    // Standard D&D
    Blessed,
    Hasted,
    Invisible,
    Sanctuary,
    
    // Resistance
    ResistancePhysical(u8),
    ResistanceMagical(u8),

    // Homebrew
    Berserker,
    RegenerationSmall,
    RegenerationGreat,
    Ascension,
    Agility,
    Swiftness,
    Endurance,
    Eclipse,
}