use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct StatLevel<T> {
    base: u8,
    #[serde(default)]
    bonus: u8,
}

impl StatLevel<T> {
    pub fn new(base: u8) -> Self {
        Self(base.clamp(1, 20), 0)
    }

    pub fn base(&self) -> u8 {
        self.base
    }

    pub fn bonus(&self) -> u8 {
        self.bonus
    }

    pub fn modify_base(&self, delta: u8) {
        self.base = (self.base + delta).clamp(1, 20);
    }

    pub fn update_bonus(&self, delta: u8) {
        self.bonus = (self.bonus + delta).clamp(0, 10);
    }

    pub fn total_value(&self) -> u8 {
        self.base + self.bonus
    }

    pub fn modifier(&self) -> u8 {
        ((self.total_value() - 10) as u8 / 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Stat<T> {
    value: StatValue,
    #[serde(skip)]
    _marker: PhantomData,
}

impl<T> Deref for Stat<T> {
    type Target = StatValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Stat<T> {
    type Target = StatValue;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}