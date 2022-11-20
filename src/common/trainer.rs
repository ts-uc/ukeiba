#![allow(unused)]
use crate::enums::Racecourse;
use chrono::{Date, Local};
use std::fmt;

struct Trainer {
    trainer_id: i32,
}

impl fmt::Display for Trainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:05}", self.trainer_id)
    }
}

impl std::str::FromStr for Trainer {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            trainer_id: s.parse().unwrap(),
        })
    }
}
