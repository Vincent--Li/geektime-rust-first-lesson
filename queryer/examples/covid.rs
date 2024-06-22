use anyhow::Result;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(url).await?.text().await?;

    // 使用 polars 直接请求

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(".\\examples\\owid-covid-latest.csv".into()))?
        .finish()?;

    let mask = df.column("new_deaths")?.gt(5);
    let filtered = df.filter(&mask?)?;
    println!(
        "{:?}",
        filtered.select([
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ])
    );
    Ok(())
}
