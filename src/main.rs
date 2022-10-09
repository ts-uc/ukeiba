#![deny(warnings)]

fn main() {
    env_logger::init();
    let _ = scraping();
}

fn scraping() -> Result<(), Box<dyn std::error::Error>> {
    let race_id: i64 = 202208200304100107;
    let url: String = format!(
        "https://keiba.rakuten.co.jp/race_performance/list/RACEID/{}",
        race_id
    );

    eprintln!("Fetching {:?}...", url);

    let res = reqwest::blocking::get(url)?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body: String = res.text()?;
    eprintln!("Body: {}", body);

    Ok(())
}
