use super::statlevel::Stat;

// These are stats that fit into the general dnd 1-20 + bonuses scheme
pub struct StrengthMarker;
pub struct AgilityMarker;
pub struct PhysiqueMarker;
pub struct IntelligenceMarker;
pub struct WisdomMarker;
pub struct CharismaMarker;
pub struct InspirationMarker;
pub struct InitialiveMarker;
pub struct PerceptionMarker;

pub type Strength = Stat<StrengthMarker>;
pub type Agility = Stat<AgilityMarker>;
pub type Physique = Stat<PhysiqueMarker>;
pub type Intelligence = Stat<IntelligenceMarker>;
pub type Wisdom = Stat<WisdomMarker>;
pub type Charisma = Stat<CharismaMarker>;
pub type Inspiration = Stat<InspirationMarker>;
pub type Initiative = Stat<InitiativeMarker>;
pub type PerceptionMarker = Stat<PerceptionMarker>;


// The rest goes here
pub struct Hits {
    hits_cap: u8,
    current_hits: u8,
    temp_hits: u8,
}

impl Hits {
    fn new(hits_cap: u8, current_hits: u8, temp_hits: u8) -> Self {
        Self(max_hits, current_hits, temp_hits)
    }

    fn hits_cap(&self) {
        self.hits_cap
    }
    
    fn set_cap(&mut self, new_cap: u8) {
        self.hits_cap = new_cap;
    }

    fn modify_cap(&mut self, delta: u8) {
        self.hits_cap = (self.hits_cap + delta).clamp;
        if self.hits_cap < 0 {
            self.hits_cap = 1;
        }
    }

}