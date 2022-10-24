#![allow(unused)]
use thiserror::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("failed to fetch from web site")]
    FetchingError,

    #[error("failed to fetch from web site")]
    NonBusinessDay,

    #[error("data store disconnected")]
    SelectorParseError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Racecourse {
    Obihiro,
    Monbetsu,
    Morioka,
    Mizusawa,
    Urawa,
    Funabashi,
    Ohi,
    Kawasaki,
    Kanazawa,
    Kasamatsu,
    Nagoya,
    Sonoda,
    Himeji,
    Kochi,
    Saga,
}

impl Racecourse {
    pub fn get_name(&self) -> String {
        match self {
            Racecourse::Monbetsu => "門別",
            Racecourse::Obihiro => "帯広",
            Racecourse::Morioka => "盛岡",
            Racecourse::Mizusawa => "水沢",
            Racecourse::Urawa => "浦和",
            Racecourse::Funabashi => "船橋",
            Racecourse::Ohi => "大井",
            Racecourse::Kawasaki => "川崎",
            Racecourse::Kanazawa => "金沢",
            Racecourse::Kasamatsu => "笠松",
            Racecourse::Nagoya => "名古屋",
            Racecourse::Sonoda => "園田",
            Racecourse::Himeji => "姫路",
            Racecourse::Kochi => "高知",
            Racecourse::Saga => "佐賀",
        }
        .to_string()
    }

    pub fn get_keibagojp_id(&self) -> i32 {
        match self {
            Racecourse::Obihiro => 3,
            Racecourse::Monbetsu => 36,
            Racecourse::Morioka => 10,
            Racecourse::Mizusawa => 11,
            Racecourse::Urawa => 18,
            Racecourse::Funabashi => 19,
            Racecourse::Ohi => 20,
            Racecourse::Kawasaki => 21,
            Racecourse::Kanazawa => 22,
            Racecourse::Kasamatsu => 23,
            Racecourse::Nagoya => 24,
            Racecourse::Sonoda => 27,
            Racecourse::Himeji => 28,
            Racecourse::Kochi => 31,
            Racecourse::Saga => 32,
        }
    }

    pub fn get_jravan_id(&self) -> i32 {
        match self {
            Racecourse::Obihiro => 33,
            Racecourse::Monbetsu => 30,
            Racecourse::Morioka => 35,
            Racecourse::Mizusawa => 36,
            Racecourse::Urawa => 42,
            Racecourse::Funabashi => 43,
            Racecourse::Ohi => 44,
            Racecourse::Kawasaki => 45,
            Racecourse::Kanazawa => 46,
            Racecourse::Kasamatsu => 47,
            Racecourse::Nagoya => 48,
            Racecourse::Sonoda => 50,
            Racecourse::Himeji => 51,
            Racecourse::Kochi => 54,
            Racecourse::Saga => 55,
        }
    }
}
