use super::uniquemodifiervalue::UniqueModifier;


pub struct Athletics;

pub struct Acrobatics;
pub struct SleightOfHand;
pub struct Stealth;

pub struct Analysis;
pub struct History;
pub struct Magic;
pub struct Nature;
pub struct Religion;

pub struct Perception;
pub struct Survival;
pub struct Medicine;
pub struct Insight;
pub struct AnimalHandling;

pub struct Presentation;
pub struct Intimidation;
pub struct Deception;
pub struct Persuasion;


pub struct StrengthModifiers {
    pub athletics: UniqueModifier<Athletics>,
}

pub struct AgilityModifiers {
    pub acrobatics: UniqueModifier<Acrobatics>,
    pub sleight_of_hand: UniqueModifier<SleightOfHand>,
    pub stealth: UniqueModifier<Stealth>,
}

pub struct IntelligenceModifiers {
    pub analysis: UniqueModifier<Analysis>,
    pub history: UniqueModifier<History>,
    pub magic: UniqueModifier<Magic>,
    pub nature: UniqueModifier<Nature>,
    pub religion: UniqueModifier<Religion>,
}

pub struct WisdomModifiers {
    pub perception: UniqueModifier<Perception>,
    pub survival: UniqueModifier<Survival>,
    pub medicine: UniqueModifier<Medicine>,
    pub insight: UniqueModifier<Insight>,
    pub animal_handling: UniqueModifier<AnimalHandling>,
}

pub struct CharismaModifiers {
    pub presentation: UniqueModifier<Presentation>,
    pub intimidation: UniqueModifier<Intimidation>,
    pub deception: UniqueModifier<Deception>,
    pub persuasion: UniqueModifier<Persuasion>,
}

pub struct NoModifiers;