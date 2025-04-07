use std::{error::Error, process::Stdio};

use thirtyfour::prelude::*;
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tucant_tests::test;

// cargo test --test chromium-extension -- --nocapture
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // .arg("--enable-chrome-logs")
    let mut child = tokio::process::Command::new("chromedriver").arg("--port=9515").kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

    let stderr = child.stdout.take().unwrap();

    let task = tokio::spawn(async {
        let mut reader = BufReader::new(stderr).lines();

        while let Some(line) = reader.next_line().await? {
            println!("{line}");
            if line == "ChromeDriver was started successfully on port 9515." {
                break;
            }
        }

        let mut caps = DesiredCapabilities::chrome();
        caps.set_no_sandbox()?;
        //caps.set_headless()?;
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        driver.set_window_rect(0, 0, 1300, 768).await?;

        test(tucant_tests::Browser::Chromium, tucant_tests::Mode::Api, driver).await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;
    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
