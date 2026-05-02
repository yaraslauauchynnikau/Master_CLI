use crate::enums::origin::{Origin, Race};
use super::essentials::{
    Strength, 
    Agility, 
    Physique, 
    Intelligence, 
    Wisdom, 
    Charisma, 
    Inspiration, 
    Initiative, 
    NaturalPerception
};

pub struct Character {
    alias: String,
    plot: String,
    origin: Origin,
    race: Race,
    //class
    //subclass
    //kind
    strength: Strength,
    agility: Agility,
    physique: Physique,
    intelligence: Intelligence,
    wisdom: Wisdom,
    charisma: Charisma,
    inspiration: Inspiration,
    initiative: Initiative,
    natural_perception: NaturalPerception,
}

impl Character {
    pub fn new<S>(
            alias: S, 
            plot: S,
            origin: Origin,
            race: Race,
            strength: Strength, 
            agility: Agility, 
            physique: Physique, 
            intelligence: Intelligence, 
            wisdom: Wisdom, 
            charisma: Charisma, 
            inspiration: Inspiration, 
            initiative: Initiative, 
            natural_perception: NaturalPerception) -> Self where S: Into<String> {
        Self{
            alias: alias.into(),
            plot: plot.into(),
            origin: origin,
            race: race,
            strength: strength,
            agility: agility,
            physique: physique,
            intelligence: intelligence,
            wisdom: wisdom,
            charisma: charisma,
            inspiration: inspiration,
            initiative: initiative,
            natural_perception: natural_perception,
        }
    }
}