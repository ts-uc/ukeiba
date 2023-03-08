#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Trainer(i64);

impl Trainer {
    pub fn new(horse_id: i64) -> Self {
        Self(horse_id)
    }

    pub fn get_id(&self) -> i64 {
        self.0
    }
}
