use std::error::Error;

use thirtyfour::prelude::*;

pub enum Browser {
    Firefox,
    Chromium,
}

pub enum Mode {
    Extension,
    Api,
}

pub async fn test(
    browser: Browser,
    mode: Mode,
    driver: WebDriver,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    driver
        .goto(match mode {
            Mode::Extension => "https://www.tucan.tu-darmstadt.de",
            Mode::Api => "http://localhost:1420",
        })
        .await?;

    assert_eq!(driver.title().await?, "TUCaN't");

    assert_eq!(
        driver.current_url().await.unwrap().scheme(),
        match browser {
            Browser::Firefox => "moz-extension",
            Browser::Chromium => "chrome-extension",
        }
    );

    driver.quit().await?;

    Ok(())
}
