// mod keibagojp;
pub mod keibagojp_racecard;
pub mod keibagojp_result;
// mod write_racecard;



//指定した日付・競馬場のデータをWebサイトから取得し、sqliteに書き込む
// pub fn scrap(from_date: Date<Local>, to_date: Date<Local>, racecourse: Racecourse) {
//     // scrap_result(&from_date, &racecourse, &7);
//     let mut date = from_date;
//     loop {
//         if to_date < date {
//             break;
//         }
//         match scrap_keibagojp(date, &racecourse) {
//             Ok(_) => (),
//             Err(CustomError::NonBusinessDay) => (),
//             Err(_) => {
//                 break;
//             }
//         };
//         date = date + chrono::Duration::days(1);
//     }
// }
