use crate::common::race::Race;

#[derive(Debug)]
pub struct PageRace {
    pub html: String,
    pub race: Race,
}

impl PageRace {
    pub fn new(html: String, race: Race) -> Self {
        Self {
            html: html,
            race: race,
        }
    }
}