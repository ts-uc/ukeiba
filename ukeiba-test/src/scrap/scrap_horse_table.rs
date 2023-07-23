use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        Dates, RaceHorses, Races,
    },
    get::get_race_data,
};
use chrono::{NaiveDate, NaiveTime};
use ukeiba_common::scraper::horse_table;

#[derive(Debug, Clone, Default)]
pub struct RaceNameInfo {
    pub race_name: Option<String>,
    pub race_detail: Option<String>,
    pub race_age: Option<String>,
    pub race_sex: Option<String>,
    pub race_class: Option<String>,
    pub race_kumi: Option<i32>,
    pub race_class_mixed: Option<bool>,
    pub race_kumi_mixed: Option<bool>,
    pub race_selected: Option<bool>,
    pub race_birthplace_limited: Option<bool>,
    pub race_final: Option<bool>,
}

pub fn scrap() {
    let race_data = get_race_data::get_all_from_db(NaiveDate::from_ymd_opt(1998, 01, 01).unwrap());
    let pages = race_data
        .into_iter()
        .map(|x| horse_table::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
            race_num: x.race_num,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        let race_name_info = split_race_name_info(&datum.race_title);
        db_writer.push(DbWriter::UpsertDates(Dates {
            race_date: datum.race_date,
            racecourse: Some(datum.racecourse.to_name()),
            ..Default::default()
        }));
        db_writer.push(DbWriter::HorseTableToRaces(Races {
            race_date: datum.race_date,
            race_num: datum.race_num,
            post_time: datum
                .post_time
                .and_then(|x| NaiveTime::parse_from_str(&x, "%H:%M").ok()),
            post_time_change: datum.post_time_change,
            race_sub_name: datum.race_sub_title,
            race_name: race_name_info.race_name,
            race_detail: race_name_info.race_detail,
            weather: match datum.race_detail.weather.as_deref() {
                Some("晴") => Some(1),
                Some("曇") => Some(2),
                Some("雨") => Some(3),
                Some("小雨") => Some(4),
                Some("雪") => Some(5),
                Some("小雪") => Some(6),
                Some(_) | None => None,
            },
            going: datum.race_detail.going.and_then(|x| x.parse().ok()),
            race_class: race_name_info.race_class,
            race_kumi: race_name_info.race_kumi,
            race_class_mixed: race_name_info.race_class_mixed,
            race_kumi_mixed: race_name_info.race_kumi_mixed,
            race_age: match (race_name_info.race_age, datum.race_detail.race_age) {
                (Some(val), _) => Some(val),
                (_, Some(val)) => Some(val),
                _ => None,
            },
            race_sex: race_name_info.race_sex,
            race_final: race_name_info.race_final,
            race_weight_type: match datum.race_detail.race_weight_type.as_deref() {
                Some("規定") => Some(1),
                Some("別定") => Some(2),
                Some("馬齢") => Some(3),
                Some("定量") => Some(4),
                Some(_) | None => None,
            },
            horse_count_entered: Some(datum.registered_horse_count),
            ..Default::default()
        }));
        for x in datum.data {
            db_writer.push(DbWriter::HorseTableToRaceHorses(RaceHorses {
                race_date: datum.race_date,
                race_num: datum.race_num,
                horse_num: x.horse_num,
                horse_nar_id: Some(x.horse_nar_id),
                bracket_num: x.bracket_num,
                horse_sex: match x.horse_sex.as_deref() {
                    Some("牡") => Some(1),
                    Some("牝") => Some(2),
                    Some("セン") => Some(3),
                    Some(_) | None => None,
                },
                jockey_nar_id: x.jockey_nar_id,
                weight_mark: match x.horse_weight_mark.as_deref() {
                    Some("△") => Some(-20),
                    Some("☆") => Some(-10),
                    Some(_) | None => Some(0),
                },
                weight_to_carry: x.weight_to_carry,
                trainer_nar_id: x.trainer_nar_id,
                owner_name: x.owner_name,
                horse_weight: x.horse_weight,
                change: x.horse_change,
                ..Default::default()
            }));
        }
    }
    write_to_db(&db_writer);
}

fn split_race_name_info(raw: &str) -> RaceNameInfo {
    let re = regex::Regex::new(
        r"(?x)
        \s*(.*?)\s*
        (
        ((?:1?\d)(?:・1?\d)?\s?[才歳](?:以?上)?(?:1?\d[才歳]以?下)?)?
        \s*(?:([牡牝雄雌])馬?)?
        \s*(勝入)?
        \s*(混\s?合)?
        \s*(?:別定)?
        (?:(オー|オープン|オープン父競走経歴馬|オールカマー|OP|A1|A2|B1|B2|B3|B4|C1|C2|C3|C4|A|B|C|D|新馬|受賞|未受賞|受賞・未受賞|受賞未受賞|新馬・受賞・未受賞|新馬未受賞|新馬・未受賞|優勝馬)(?:[-ー](\d+))?(?:・(?:(?:A1|A2|B1|B2|B3|B4|C1|C2|C3|C4)(?:-\d+)?|(\d+))(?:・\d+)?)?)?
        (?:(\d{1,3}0)万?円?未?満?)?
        \s*
        (?:
        ([青栗鹿芦]毛馬?選抜|産駒特別(?:競走)?選抜|(?:オープン)?[高重]馬体重馬?(?:選抜)?|(?:指定)?(?:重賞)?・?(?:特別)?競走優勝馬?|騎手指定選抜|指定選抜|トライアル選抜|馬齢選抜|ファン投票選抜|ヤングジョッキー選抜|選\s?抜)|
        ((?:産地限定)?(?:足寄町?|網走|石狩後志渡島桧山胆振日高|石狩空知後志上川留萌宗谷|渡島桧山胆振日高|釧路・?(?:根室)?|空知上川留萌宗谷|大樹町|十勝|南北海道)(?:育成馬)?(?:管内)?(?:産駒)?(?:選抜)?|産地限定|駒選抜)
        )?
        \s*(決勝|勝入)?
        \s*(混\s?合?)?
        (?:-\d+(?:・\d+)?)?
        \s*(?:別定定量|別\s?定|定\s?量|馬齢)?
    )\s*$
        "
    ).unwrap();
    if let Some(captures) = re.captures(raw) {
        let race_name = captures
            .get(1)
            .map(|m| m.as_str().to_string())
            .filter(|s| !s.is_empty());
        let race_detail = captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .filter(|s| !s.is_empty());
        let race_age = captures
            .get(3)
            .map(|m| {
                m.as_str()
                    .replace("才", "歳")
                    .replace("歳上", "歳以上")
                    .replace("歳下", "歳以下")
                    .replace(" ", "")
            })
            .filter(|s| !s.is_empty());
        let race_sex = match captures.get(4).map(|m| m.as_str()) {
            Some("雄") | Some("牡") => Some("牡馬".to_string()),
            Some("雌") | Some("牝") => Some("牝馬".to_string()),
            _ => None,
        };
        let is_final_before = captures.get(5).is_some();
        let is_mixed_before = captures.get(6).is_some();
        let race_class_current = captures
            .get(7)
            .map(|m| m.as_str().to_string())
            .filter(|s| !s.is_empty());
        let race_kumi = captures.get(8).and_then(|m| m.as_str().parse::<i32>().ok());
        let race_kumi_mixed = captures.get(9).is_some();
        let race_class_old = captures
            .get(10)
            .map(|m| m.as_str().to_string())
            .filter(|s| !s.is_empty());
        let is_selected = captures.get(11).is_some();
        let birthplace_limited = captures.get(12).is_some();
        let is_final_after = captures.get(13).is_some();
        let is_mixed_after = captures.get(14).is_some();

        return RaceNameInfo {
            race_name: race_name,
            race_detail: race_detail,
            race_age: race_age,
            race_sex: race_sex,
            race_class: match (race_class_current, race_class_old) {
                (Some(val), _) => Some(val),
                (_, Some(val)) => Some(val),
                _ => None,
            },
            race_kumi: race_kumi,
            race_class_mixed: Some(is_mixed_before || is_mixed_after),
            race_kumi_mixed: Some(race_kumi_mixed),
            race_selected: Some(is_selected),
            race_birthplace_limited: Some(birthplace_limited),
            race_final: Some(is_final_before || is_final_after),
        };
    }

    RaceNameInfo {
        ..Default::default()
    }
}
