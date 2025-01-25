use std::{error::Error, process::Stdio};

use thirtyfour::{extensions::addons::firefox::FirefoxTools, prelude::*};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tucant_tests::test;

// geckodriver --binary firefox-nightly
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut child = tokio::process::Command::new("geckodriver")
        .arg("--binary=firefox-nightly")
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .spawn()?;

    let stderr = child.stdout.take().unwrap();

    let task = tokio::spawn(async {
        let mut reader = BufReader::new(stderr).lines();

        while let Some(line) = reader.next_line().await? {
            println!("{line}");
            if line.contains("Listening on") {
                break;
            }
        }

        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", caps).await?;
        let tools = FirefoxTools::new(driver.handle.clone());
        tools
            .install_addon(&std::env::var("EXTENSION_DIR").unwrap(), Some(true))
            .await?;

        test(
            tucant_tests::Browser::Firefox,
            tucant_tests::Mode::Extension,
            driver,
        )
        .await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;
    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
