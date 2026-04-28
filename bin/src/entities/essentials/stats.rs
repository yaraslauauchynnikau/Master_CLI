use std::thread::current;

use super::statvalue::{Stat, HasUniqueModifiers};
use super::uniquemodifiers::*;
// These are stats that fit into the general dnd 1-20 + bonuses scheme
pub struct StrengthMarker;
pub struct AgilityMarker;
pub struct PhysiqueMarker;
pub struct IntelligenceMarker;
pub struct WisdomMarker;
pub struct CharismaMarker;
pub struct InspirationMarker;
pub struct InitiativeMarker;
pub struct PerceptionMarker;

impl HasUniqueModifiers for StrengthMarker { type UniqueModifiers = StrengthModifiers; }
impl HasUniqueModifiers for AgilityMarker { type UniqueModifiers = AgilityModifiers; }
impl HasUniqueModifiers for PhysiqueMarker { type UniqueModifiers = NoModifiers; }
impl HasUniqueModifiers for IntelligenceMarker { type UniqueModifiers = IntelligenceModifiers; }
impl HasUniqueModifiers for WisdomMarker { type UniqueModifiers = WisdomModifiers; }
impl HasUniqueModifiers for CharismaMarker { type UniqueModifiers = CharismaModifiers; }

pub type Strength = Stat<StrengthMarker>;
pub type Agility = Stat<AgilityMarker>;
pub type Physique = Stat<PhysiqueMarker>;
pub type Intelligence = Stat<IntelligenceMarker>;
pub type Wisdom = Stat<WisdomMarker>;
pub type Charisma = Stat<CharismaMarker>;
pub type Inspiration = Stat<InspirationMarker>;
pub type Initiative = Stat<InitiativeMarker>;
pub type Perception = Stat<PerceptionMarker>;


// The rest goes here

pub struct Hits {
    hits_cap: u8,
    current_hits: u8,
    temp_hits: u8,
}

impl Hits {
    fn new(hits_cap: u8, current_hits: u8, temp_hits: u8) -> Self {
        Self {
            hits_cap: hits_cap,
            current_hits: current_hits,
            temp_hits: temp_hits,
        }
    }

    fn hits_cap(&self) -> u8 {
        self.hits_cap
    }
    
    fn set_cap(&mut self, new_cap: u8) {
        self.hits_cap = new_cap;
    }

    fn modify_cap(&mut self, delta: u8) {
        self.hits_cap = (self.hits_cap + delta);
        if self.hits_cap < 0 {
            self.hits_cap = 1;
        }
    }

}