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
pub struct NaturalPerceptionMarker;

impl HasUniqueModifiers for StrengthMarker { type UniqueModifiers = StrengthModifiers; }
impl HasUniqueModifiers for AgilityMarker { type UniqueModifiers = AgilityModifiers; }
impl HasUniqueModifiers for PhysiqueMarker { type UniqueModifiers = NoModifiers; }
impl HasUniqueModifiers for IntelligenceMarker { type UniqueModifiers = IntelligenceModifiers; }
impl HasUniqueModifiers for WisdomMarker { type UniqueModifiers = WisdomModifiers; }
impl HasUniqueModifiers for CharismaMarker { type UniqueModifiers = CharismaModifiers; }
impl HasUniqueModifiers for InitiativeMarker { type UniqueModifiers = NoModifiers; }
impl HasUniqueModifiers for NaturalPerceptionMarker { type UniqueModifiers = NoModifiers; }
impl HasUniqueModifiers for InspirationMarker { type UniqueModifiers = NoModifiers; }

pub type Strength = Stat<StrengthMarker>;
pub type Agility = Stat<AgilityMarker>;
pub type Physique = Stat<PhysiqueMarker>;
pub type Intelligence = Stat<IntelligenceMarker>;
pub type Wisdom = Stat<WisdomMarker>;
pub type Charisma = Stat<CharismaMarker>;
pub type Inspiration = Stat<InspirationMarker>;
pub type Initiative = Stat<InitiativeMarker>;
pub type NaturalPerception = Stat<NaturalPerceptionMarker>;


// The rest goes here
pub struct Hits {
    cap: u8,
    current_hits: u8,
    temp_hits: u8,
}

impl Hits {
    fn new(cap: u8, current_hits: u8, temp_hits: u8) -> Self {
        Self {
            cap: cap,
            current_hits: current_hits,
            temp_hits: temp_hits,
        }
    }

    fn cap(&self) -> u8 {
        self.cap
    }
    
    fn set_cap(&mut self, new_cap: u8) {
        self.cap = new_cap;
    }

    fn modify_cap(&mut self, delta: i8) {
        self.cap = (self.cap as i8 + delta).max(1) as u8;
    }

    fn current_hits(&self) -> u8 {
        self.current_hits
    }

    fn set_current_hits(&mut self, new_hits: u8) {
        self.current_hits = new_hits;
    }

    fn hurt(&mut self, damage: u8) {
        if damage > self.temp_hits {
            if damage > self.temp_hits + self.current_hits {
                self.current_hits = 0;
            } else {
                self.current_hits = self.current_hits - damage + self.temp_hits;
            }
            self.temp_hits = 0;    
        } else {
            self.temp_hits -= damage;
        }
    }

    fn heal(&mut self, amount: u8) {
        self.current_hits = (self.current_hits + amount).clamp(0, self.cap)
    }
}

pub struct Speed {
    value: i8,
    cap: i8
}

impl Speed {
    pub fn new(value: i8, cap: i8) -> Self {
        Self {
            value: value,
            cap: cap,
        }
    }

    pub fn set_value(&mut self, new_value: i8) {
        self.value = new_value;
    }

    pub fn modify_value(&mut self, delta: i8) {
        self.value = (self.value + delta).clamp(0, self.cap);
    }
}