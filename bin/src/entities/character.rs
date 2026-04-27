use origin::{Race, Origin};

pub struct Character {
    alias: String,
    player_name: Option<String>,
    class: Class,
    race: Race,
    origin: Origin,
    world_view: WorldView,
    experience: u32,


    strength_level: u8,
    strength_modifier: u8,

    agility_level: u8,
    agility_modifier: u8,

    physique_level: u8,
    physique_modifier: u8,

    intelligence_level: u8,
    intelligence_modifier: u8,

    wisdom_level: u8,
    wisdom_modifier: u8,

    charisma_level: u8,
    charisma_modifier: u8,

    inspiration: Option<String>,

    initiative: u8,
    initiative_modifier: u8,

    perception: u8,

    speed: u8,


    max_hits: u8,
    current_hits: u8,
    temporary_hits: u8,

    death_saverolls_successful: u8,
    death_saverolls_failed: u8,


    weapons: Option<Vec<Weapon>>,
    traits: Option<Vec<String>>,
    ideals: Option<Vec<String>>,
    attachments: Option<Vec<String>>,
    weaknesses: Option<Vec<String>>,

    
}