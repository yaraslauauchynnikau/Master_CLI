use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

const BASE_CAP: u8 = 20;
const BONUS_CAP: u8 = 10;

pub trait HasModifiers {
    type Modifiers;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct StatValue {
    base: u8,
    #[serde(default)]
    bonus: u8,
}

impl StatValue {
    pub fn new(base: u8) -> Self {
        Self(base.clamp(1, BASE_CAP), 0)
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
        ((self.total_value() - 10) as i8 / 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Stat<T: HasModifiers> {
    value: StatValue,
    pub modifiers: T,
    
    #[serde(skip)]
    _marker: PhantomData,
}

impl<T: HasModifiers> Deref for Stat<T: HasModifiers> {
    type Target = StatValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: HasModifiers> DerefMut for Stat<T: HasModifiers> {
    type Target = StatValue;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}