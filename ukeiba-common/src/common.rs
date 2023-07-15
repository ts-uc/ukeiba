use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum HorseBelong {
    #[default]
    Left = 0,
    Banei = 21,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Racecourse {
    #[default]
    Other,
    Kitami,
    Iwamizawa,
    Obihiro,
    Asahikawa,
}

impl Racecourse {
    pub fn from_name(name: &str) -> Self {
        match name {
            "北見" => Self::Kitami,
            "岩見沢" => Self::Iwamizawa,
            "帯広" => Self::Obihiro,
            "旭川" => Self::Asahikawa,
            _ => Self::Other,
        }
    }
    pub fn to_name(&self) -> String {
        match self {
            Self::Other => "".to_string(),
            Self::Kitami => "北見".to_string(),
            Self::Iwamizawa => "岩見沢".to_string(),
            Self::Obihiro => "帯広".to_string(),
            Self::Asahikawa => "旭川".to_string(),
        }
    }
    pub fn to_nar_id(&self) -> i32 {
        match self {
            Self::Other => 0,
            Self::Kitami => 1,
            Self::Iwamizawa => 2,
            Self::Obihiro => 3,
            Self::Asahikawa => 4,
        }
    }
    pub fn to_oddspark_id(&self) -> i32 {
        match self {
            Self::Other => 0,
            Self::Kitami => 1,
            Self::Iwamizawa => 2,
            Self::Obihiro => 3,
            Self::Asahikawa => 5,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceData {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
    pub race_num: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRacecourse {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HorseBajikyoId(pub String);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HorseNarId(pub i64);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JockeyNarId(pub i32);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainerNarId(pub i32);
