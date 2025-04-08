use std::{error::Error, time::Duration};

use thirtyfour::prelude::*;
use tokio::time::sleep;

pub enum Browser {
    Firefox,
    Chromium,
}

pub enum Mode {
    Extension,
    Api,
}

pub async fn test(browser: Browser, mode: Mode, driver: WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().unwrap();

    sleep(Duration::from_secs(1)).await; // wait for extension to be installed

    driver
        .goto(match mode {
            Mode::Extension => "https://www.tucan.tu-darmstadt.de/",
            Mode::Api => "http://localhost:1420",
        })
        .await?;

    assert_eq!(driver.title().await?, "TUCaN't");

    assert_eq!(
        driver.current_url().await.unwrap().scheme(),
        match mode {
            Mode::Extension => match browser {
                Browser::Firefox => "moz-extension",
                Browser::Chromium => "chrome-extension",
            },
            Mode::Api => "http",
        }
    );

    let username_input = driver.query(By::Css("#login-username")).first().await?;
    let password_input = driver.find(By::Css("#login-password")).await?;
    let login_button = driver.find(By::Css("#login-button")).await?;

    let username = std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing");
    let password = std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing");

    username_input.send_keys(username).await?;
    password_input.send_keys(password).await?;
    // probably https://yew.rs/docs/concepts/html/events#event-delegation
    username_input.focus().await?;
    login_button.click().await?;

    sleep(Duration::from_secs(10)).await;

    driver.quit().await?;

    Ok(())
}

macro_rules! all_browsers {
    ($function_name: ident) => {
        ::paste::paste! {
            #[::tokio::test]
            pub async fn [<$function_name _firefox>]() {

            }
        }
    };
}

all_browsers!(test);
