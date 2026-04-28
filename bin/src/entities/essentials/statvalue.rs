use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;


const BASE_CAP: u8 = 80;
const BONUS_CAP: u8 = 80;

pub trait HasUniqueModifiers {
    type UniqueModifiers;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatValue {
    base: u8,
    #[serde(default)]
    bonus: u8,
}

impl StatValue {
    pub fn new(base: u8) -> Self {
        Self {
            base: base.clamp(1, BASE_CAP), 
            bonus: 0
        }
    }

    pub fn base(&self) -> u8 {
        self.base
    }

    pub fn bonus(&self) -> u8 {
        self.bonus
    }

    pub fn modify_base(&mut self, delta: u8) {
        self.base = (self.base + delta).clamp(1, BASE_CAP);
    }

    pub fn update_bonus(&mut self, delta: u8) {
        self.bonus = (self.bonus + delta).clamp(0, BONUS_CAP);
    }

    pub fn total_value(&self) -> u8 {
        self.base + self.bonus
    }

    pub fn modifier(&self) -> i8 {
        (self.total_value() - 10) as i8 / 2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stat<T: HasUniqueModifiers> {
    value: StatValue,
    pub modifiers: T,
    
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T: HasUniqueModifiers> Deref for Stat<T> {
    type Target = StatValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: HasUniqueModifiers> DerefMut for Stat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}