use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Affliction {
    // Official D&D
    Blinded,
    Charmed,
    Deafened,
    Exhaustion(u8),         // 1-6
    Frightened,
    Grappled,
    Incapacitated,
    Paralyzed,
    Petrified,
    Poisoned, 
    Prone,
    Restrained,
    Stunned,
    Unconscious,

    // Homebrew
    BleedingLight,
    BleedingHard,
    Bloodcurse,
    Bloodloss,
    Curse,
    DeathMark,
    DemonicPossession,
    EclipseMark,
    InfernalTerror,
    Madness,
    Mindless,
    Mutation,
    Necrotic,
    Possession,
    RottingFlesh,
}