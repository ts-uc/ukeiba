use std::fmt;

struct Horse {
    horse_id: i64,
}

impl fmt::Display for Horse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:011}", self.horse_id)
    }
}

impl std::str::FromStr for Horse {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            horse_id: s.parse().unwrap(),
        })
    }
}
