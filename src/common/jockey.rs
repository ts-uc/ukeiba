use std::fmt;

struct Jockey{
    jockey_id: i32
}

impl fmt::Display for Jockey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:05}", self.jockey_id)
    }
}

impl std::str::FromStr for Jockey {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            jockey_id: s.parse().unwrap(),
        })
    }
}
