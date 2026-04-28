use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;


const MODIFIER_CAP: i8 = 80;
const MODIFIER_CEILING: i8 = -5;

const ENHANCEMENT_CAP: i8 = 5;
const ENHANCEMENT_CEILING: i8 = -5;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueModifierValue {
    enhancement_level: i8,
}


impl UniqueModifierValue {
    pub fn new(enhancement_level: i8) -> Self {
        Self {
            enhancement_level: enhancement_level.clamp(ENHANCEMENT_CEILING, ENHANCEMENT_CAP),
        }
    }

    pub fn enhancement_level(&self) -> i8 {
        self.enhancement_level
    }

    pub fn set_enhancement_level(&mut self, new_level: i8) {
        self.enhancement_level = new_level.clamp(ENHANCEMENT_CEILING, ENHANCEMENT_CAP);
    }

    pub fn modify_enhancement_level(&mut self, delta: i8) {
        self.enhancement_level = (self.enhancement_level + delta).clamp(ENHANCEMENT_CEILING, ENHANCEMENT_CAP);
    }

    pub fn value(&self) -> i8 {
        self.enhancement_level * 2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UniqueModifier<T> {
    value: UniqueModifierValue,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Deref for UniqueModifier<T> {
    type Target = UniqueModifierValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for UniqueModifier<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}