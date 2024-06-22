use std::{error::Error, thread::sleep, time::Duration};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--user-data-dir=PATH/TO/YOUR/USER/DATA")
        .unwrap();
    caps.add_arg("--no-sandbox").unwrap();
    caps.add_arg("--disable-gpu").unwrap();
    // caps.add_arg("--disable-dev-shm-usage").unwrap();
    caps.add_experimental_option("debuggerAddress", "127.0.0.1:9527")
        .unwrap();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://time.geekbang.org/column/article/414478")
        .await?;
    sleep(Duration::from_secs(10));
    let links = driver
        .find_all(By::XPath("//a[@data-slate-type='link']"))
        .await?;
    for link in links {
        link.click().await?;
    }
    sleep(Duration::from_secs(10));
    for handle in driver.windows().await? {
        println!("{}", handle.to_string())
    }

    // Find element from element.
    // let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    // elem_text.send_keys("selenium").await?;

    // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.query(By::ClassName("firstHeading")).first().await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser.
    // driver.quit().await?;

    Ok(())
}
