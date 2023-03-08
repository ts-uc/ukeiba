#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Jockey(i64);

impl Jockey {
    pub fn new(horse_id: i64) -> Self {
        Self(horse_id)
    }

    pub fn get_id(&self) -> i64 {
        self.0
    }
}
