extern crate ukeiba_scraper;
use anyhow::Result;
use chrono::NaiveDate;
use csv::Writer;
use serde::Serialize;
use std::fs::File;
use ukeiba_scraper::{Mode, WebPageTrait};

#[derive(Debug, Clone, Serialize)]
struct HorseData {
    pub horse_nar_id: i64,
    pub horse_bajikyo_id: String,
    pub horse_name: String,
    pub horse_sex: String,
    pub horse_status: String,
    pub horse_type: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub sire_name: Option<String>,
    pub dam_name: Option<String>,
}

fn main() {
    sub()
}

fn sub() {
    let mut a = Vec::new();
    for year in (1969..=2021).rev() {
        println!("{}", year);
        let page_data = ukeiba_scraper::horse_search::Page {
            page_num: 1,
            horse_name: "".to_string(),
            horse_belong: ukeiba_scraper::horse_search::HorseBelong::Banei,
            birth_year: year,
        }
        .scrap(Mode::NormalSave, std::time::Duration::from_secs(1))
        .unwrap();
        println!("{} {}", page_data.hits, year);
        if page_data.hits > 0 {
            let pages = (page_data.hits - 1) / 50 + 1;
            for page in 1..=pages {
                let page_data = ukeiba_scraper::horse_search::Page {
                    page_num: page,
                    horse_name: "".to_string(),
                    horse_belong: ukeiba_scraper::horse_search::HorseBelong::Banei,
                    birth_year: year,
                }
                .scrap(Mode::NormalSave, std::time::Duration::from_secs(1))
                .unwrap();
                let horses = page_data.data.iter().map(|x| x.horse_nar_id);
                for horse_nar_id in horses {
                    match get_horse_profile(horse_nar_id) {
                        Some(ht) => {
                            a.push(ht);
                        }
                        None => continue,
                    }
                }
            }
        }
    }

    for year in (1969..=2021).rev() {
        for kana in "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ".chars() {
            let page_data = ukeiba_scraper::horse_search::Page {
                page_num: 1,
                horse_name: kana.to_string(),
                horse_belong: ukeiba_scraper::horse_search::HorseBelong::Left,
                birth_year: year,
            }
            .scrap(Mode::NormalSave, std::time::Duration::from_secs(1))
            .unwrap();
            println!("{} {} {}",page_data.hits, kana, year);
            if page_data.hits > 0 {
                let pages = (page_data.hits - 1) / 50 + 1;
                for page in 1..=pages {
                    let page_data = ukeiba_scraper::horse_search::Page {
                        page_num: page,
                        horse_name: kana.to_string(),
                        horse_belong: ukeiba_scraper::horse_search::HorseBelong::Left,
                        birth_year: year,
                    }
                    .scrap(Mode::NormalSave, std::time::Duration::from_secs(1))
                    .unwrap();
                    let horses = page_data.data.iter().map(|x| x.horse_nar_id);
                    for horse_nar_id in horses {
                        match get_horse_profile(horse_nar_id) {
                            Some(ht) => {a.push(ht);},
                            None => continue,
                        }
                    }
                }
            }
        }
    }
    write_csv("horses.csv", &a).unwrap();
}

//3659958

fn get_horse_profile(horse_nar_id: i64) -> Option<HorseData> {
    let data = ukeiba_scraper::horse_profile::Page {
        horse_nar_id: horse_nar_id,
    }
    .scrap(Mode::NormalSave, std::time::Duration::from_secs(1))
    .ok()?;

    match data.horse_type.as_deref() {
        Some("(アア)") | Some("(サラ系)") | None => return None,
        _ => (),
    }

    Some(HorseData {
        horse_nar_id: horse_nar_id,
        horse_bajikyo_id: to_bajikyo_id(horse_nar_id),
        horse_name: data.horse_name,
        horse_sex: data.horse_sex,
        horse_status: data.horse_status,
        horse_type: data.horse_type,
        birthdate: data.birthdate,
        sire_name: data.sire_name,
        dam_name: data.dam_name,
    })
}

fn write_csv<T>(filename: &str, data: &[T]) -> Result<()>
where
    T: Serialize,
{
    let mut writer = Writer::from_path(filename)?;

    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

fn to_bajikyo_id(nar_id: i64) -> String {
    let chars: Vec<char> = nar_id.to_string().chars().collect();
    let shuffled: i64 = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        chars[5],
        chars[1],
        chars[10],
        chars[9],
        chars[2],
        chars[0],
        chars[4],
        chars[8],
        chars[3],
        chars[7]
    )
    .parse()
    .unwrap();
    let mut num_chars: Vec<char> = (shuffled - 2046971875).to_string().chars().rev().collect();

    if num_chars.len() >= 5 {
        if num_chars[4] == '5' {
            num_chars[4] = ' ';
        } else if num_chars[4] == '4' {
            num_chars[4] = 'H';
        }
    }

    num_chars.into_iter().rev().collect()
}
