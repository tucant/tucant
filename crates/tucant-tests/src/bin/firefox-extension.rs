use std::{error::Error, process::Stdio, time::Duration};

use thirtyfour::{extensions::addons::firefox::FirefoxTools, prelude::*};
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    time::sleep,
};
use tucant_tests::test;

// geckodriver --binary firefox-nightly
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut child = tokio::process::Command::new("geckodriver").kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

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
        driver.set_window_rect(0, 0, 1300, 768).await?;
        let tools = FirefoxTools::new(driver.handle.clone());
        tools.install_addon(&std::env::var("EXTENSION_DIR").unwrap(), Some(true)).await?;

        sleep(Duration::from_secs(2)).await; // wait for extension?

        test(tucant_tests::Browser::Firefox, tucant_tests::Mode::Extension, driver).await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;
    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
