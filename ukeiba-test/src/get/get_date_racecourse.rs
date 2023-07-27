use chrono::{Duration, Local, NaiveDate};
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

pub fn get_all_available() -> Vec<DateRacecourse> {
    let mut date_racecourses: Vec<DateRacecourse> = Vec::new();

    // Kitami
    let start_date = NaiveDate::from_ymd_opt(1998, 06, 06).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2006, 11, 27).unwrap();
    let dates = start_date.iter_days().take_while(|x| x <= &end_date);
    date_racecourses.extend(dates.map(|x| DateRacecourse {
        race_date: x,
        racecourse: Racecourse::Kitami,
    }));

    // Iwamizawa
    let start_date = NaiveDate::from_ymd_opt(1998, 07, 18).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2006, 10, 02).unwrap();
    let dates = start_date.iter_days().take_while(|x| x <= &end_date);
    date_racecourses.extend(dates.map(|x| DateRacecourse {
        race_date: x,
        racecourse: Racecourse::Iwamizawa,
    }));

    // Asahikawa
    let start_date = NaiveDate::from_ymd_opt(1998, 04, 18).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2006, 06, 12).unwrap();
    let dates = start_date.iter_days().take_while(|x| x <= &end_date);
    date_racecourses.extend(dates.map(|x| DateRacecourse {
        race_date: x,
        racecourse: Racecourse::Asahikawa,
    }));

    // Obihiro
    let start_date = NaiveDate::from_ymd_opt(1998, 01, 02).unwrap();
    let end_date = Local::now().naive_local().date() - Duration::days(1);
    let dates = start_date.iter_days().take_while(|x| x <= &end_date);
    date_racecourses.extend(dates.map(|x| DateRacecourse {
        race_date: x,
        racecourse: Racecourse::Obihiro,
    }));

    date_racecourses
}
