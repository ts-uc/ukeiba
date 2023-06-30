use chrono::NaiveDate;
use ukeiba_common::common::{RaceData, Racecourse};

use crate::db::make_conn;

pub fn get_after_1997_01_from_db() -> Vec<RaceData> {
    let conn = make_conn().unwrap();

    let query = "
    SELECT dates.race_date, dates.racecourse, races.race_num
    FROM dates
    JOIN races ON dates.race_date = races.race_date
    WHERE dates.race_date >= '1998-01-01' AND dates.capability_test IS NULL
    ORDER BY dates.race_date ASC;
    ";

    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(RaceData {
                race_date: NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d")
                    .unwrap(),
                racecourse: Racecourse::from_name(&row.get::<_, String>(1)?),
                race_num: row.get(2)?,
            })
        })
        .unwrap();

    let race_data: Vec<_> = rows.map(|row| row.unwrap()).collect();
    race_data
}
