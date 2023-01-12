#![allow(dead_code)]
use strum_macros::EnumIter;
use clap::ValueEnum;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter, ValueEnum)]
pub enum Racecourse {
    Kitami = 31,
    Iwamizawa = 32,
    Obihiro = 33,
    Asahikawa = 34,
    // Monbetsu = 30,
    // Morioka = 35,
    // Mizusawa = 36,
    // Urawa = 42,
    // Funabashi = 43,
    // Ohi = 44,
    // Kawasaki = 45,
    // Kanazawa = 46,
    // Kasamatsu = 47,
    // Nagoya = 48,
    // Sonoda = 50,
    // Himeji = 51,
    // Kochi = 54,
    // Saga = 55,
}

impl fmt::Display for Racecourse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Racecourse {
    pub fn from_jravan_id(id: i32) -> Self{
        match id{
            31 => Self::Kitami,
            32 => Self::Iwamizawa,
            33 => Self::Obihiro,
            34 => Self::Asahikawa,
            _ => panic!()
        }
    }

    pub fn from_str(name: &str) -> Self{
        match name{
            "北見" => Self::Kitami,
            "岩見沢" => Self::Iwamizawa,
            "帯広" => Self::Obihiro,
            "旭川" => Self::Asahikawa,
            _ => panic!()
         } }

    pub fn get_name(&self) -> String {
        match self {
            // Racecourse::Monbetsu => "門別",
            Racecourse::Kitami => "北見",
            Racecourse::Iwamizawa => "岩見沢",
            Racecourse::Obihiro => "帯広",
            Racecourse::Asahikawa => "旭川",
            // Racecourse::Morioka => "盛岡",
            // Racecourse::Mizusawa => "水沢",
            // Racecourse::Urawa => "浦和",
            // Racecourse::Funabashi => "船橋",
            // Racecourse::Ohi => "大井",
            // Racecourse::Kawasaki => "川崎",
            // Racecourse::Kanazawa => "金沢",
            // Racecourse::Kasamatsu => "笠松",
            // Racecourse::Nagoya => "名古屋",
            // Racecourse::Sonoda => "園田",
            // Racecourse::Himeji => "姫路",
            // Racecourse::Kochi => "高知",
            // Racecourse::Saga => "佐賀",
        }
        .to_string()
    }

    pub fn get_keibagojp_id(&self) -> i32 {
        match self {
            Racecourse::Kitami => 1,
            Racecourse::Iwamizawa => 2,
            Racecourse::Obihiro => 3,
            Racecourse::Asahikawa => 4,
            // Racecourse::Monbetsu => 36,
            // Racecourse::Morioka => 10,
            // Racecourse::Mizusawa => 11,
            // Racecourse::Urawa => 18,
            // Racecourse::Funabashi => 19,
            // Racecourse::Ohi => 20,
            // Racecourse::Kawasaki => 21,
            // Racecourse::Kanazawa => 22,
            // Racecourse::Kasamatsu => 23,
            // Racecourse::Nagoya => 24,
            // Racecourse::Sonoda => 27,
            // Racecourse::Himeji => 28,
            // Racecourse::Kochi => 31,
            // Racecourse::Saga => 32,
        }
    }

    pub fn get_oddspark_id(&self) -> i32 {
        match self {
            Racecourse::Kitami => 1,
            Racecourse::Iwamizawa => 2,
            Racecourse::Obihiro => 3,
            Racecourse::Asahikawa => 5,
            // Racecourse::Monbetsu => 0,
            // Racecourse::Morioka => 0,
            // Racecourse::Mizusawa => 0,
            // Racecourse::Urawa => 0,
            // Racecourse::Funabashi => 0,
            // Racecourse::Ohi => 0,
            // Racecourse::Kawasaki => 0,
            // Racecourse::Kanazawa => 0,
            // Racecourse::Kasamatsu => 0,
            // Racecourse::Nagoya => 0,
            // Racecourse::Sonoda => 0,
            // Racecourse::Himeji => 0,
            // Racecourse::Kochi => 0,
            // Racecourse::Saga => 0,
        }
    }
}

impl std::str::FromStr for Racecourse {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "31" => Racecourse::Kitami,
            "32" => Racecourse::Iwamizawa,
            "33" => Racecourse::Obihiro,
            "34" => Racecourse::Asahikawa,
            // "30" => Racecourse::Monbetsu,
            // "35" => Racecourse::Morioka,
            // "36" => Racecourse::Mizusawa,
            // "42" => Racecourse::Urawa,
            // "43" => Racecourse::Funabashi,
            // "44" => Racecourse::Ohi,
            // "45" => Racecourse::Kawasaki,
            // "46" => Racecourse::Kanazawa,
            // "47" => Racecourse::Kasamatsu,
            // "48" => Racecourse::Nagoya,
            // "50" => Racecourse::Sonoda,
            // "51" => Racecourse::Himeji,
            // "54" => Racecourse::Kochi,
            // "55" => Racecourse::Saga,
            _ => panic!(),
        })
    }
}
