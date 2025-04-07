use std::{error::Error, process::Stdio, time::Duration};

use thirtyfour::prelude::*;
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    time::sleep,
};
use tucant_tests::test;

// cargo run --bin chromium-extension
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
        caps.add_arg(&format!("--load-extension={}", std::env::var("EXTENSION_DIR").unwrap()))?;
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        sleep(Duration::from_secs(2)).await; // wait for extension?

        test(tucant_tests::Browser::Chromium, tucant_tests::Mode::Extension, driver).await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;
    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
