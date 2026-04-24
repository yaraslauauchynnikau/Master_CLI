use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Serialize, Deserialize, Debug)]
pub enum Affliction {
    BERSERKER,
    BLEEDING_LIGHT,
    BLEEDING_HARD,
    BLOODCURSE,
    BLOODLOSS,
    CHAOS_MARKED,
    CURSED,
    DEATH_MARKED,
    DEATH_WISH,
    DEMONIC_POSSESSION,
    ECLIPSE_MARKED,
    FLESH_ROTTING,
    INFERNAL_TERROR,
    MADNESS,
    MINDLESS,
    MUTATED,
    NECROTIC,
    PARALYZED,
    POISONED,
    POSSESSED,
}