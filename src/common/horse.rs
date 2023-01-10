#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Horse(i64);

impl Horse{
    pub fn new(horse_id: i64) -> Self{
        Self(horse_id)
    }

    pub fn get_horse_id(&self) -> i64{
        self.0
    }

    pub fn get_upper_id(&self) -> i64{
        self.0 / 100000
    }

    pub fn get_lower_id(&self) -> i64{
        self.0 % 100000
    }
}