use chrono::NaiveDate;
use ukeiba_common::common::{DateRacecourse, Racecourse};

use crate::db::make_conn;

pub fn get_all_from_db(from: NaiveDate) -> Vec<DateRacecourse> {
    let conn = make_conn().unwrap();

    let query = format!(
        "
    SELECT race_date, racecourse
    FROM dates
    WHERE race_date >= '{}' AND capability_test IS NULL
    ORDER BY race_date ASC;
    ",
        from
    );

    let mut stmt = conn.prepare(&query).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(DateRacecourse {
                race_date: NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d")
                    .unwrap(),
                racecourse: Racecourse::from_name(&row.get::<_, String>(1)?),
            })
        })
        .unwrap();

    let race_data: Vec<_> = rows.map(|row| row.unwrap()).collect();
    race_data
}
