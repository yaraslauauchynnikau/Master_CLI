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
    natural_perception: NaturalPerception
}