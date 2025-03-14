use std::{error::Error, process::Stdio};

use thirtyfour::prelude::*;
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tucant_tests::test;

// cargo test --test chromium-extension -- --nocapture
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut child = tokio::process::Command::new("chromedriver").arg("--port=9515").arg("--enable-chrome-logs").kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

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
        caps.add_arg(&format!("--load-extension={}", std::env::var("EXTENSION_DIR").unwrap()))?;
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        test(tucant_tests::Browser::Chromium, tucant_tests::Mode::Extension, driver).await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;
    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
