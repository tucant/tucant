use regex::Regex;
use reqwest::header::HeaderValue;
use scraper::Html;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{MyClient, TucanError};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub id: u64,
    pub cookie_cnsc: String,
}

pub async fn login(
    client: &MyClient,
    login_request: &LoginRequest,
) -> Result<LoginResponse, TucanError> {
    assert_ne!(login_request.username, "");
    assert_ne!(login_request.password, "");
    let mut response = client
        .post("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll")
        .form(&[
            ("usrname", login_request.username.as_str()),
            ("pass", login_request.password.as_str()),
            ("APPNAME", "CampusNet"),
            ("PRGNAME", "LOGINCHECK"),
            (
                "ARGUMENTS",
                "clino,usrname,pass,menuno,menu_type,browser,platform",
            ),
            ("clino", "000000000000001"),
            ("menuno", "000344"),
            ("menu_type", "classic"),
            ("browser", ""),
            ("platform", ""),
        ])
        .send()
        .await?
        .error_for_status()?;
    assert_eq!(
        response.headers_mut().remove("content-type"),
        Some(HeaderValue::from_static("text/html"))
    );
    assert_eq!(
        response.headers_mut().remove("server"),
        Some(HeaderValue::from_static("Microsoft-IIS/10.0"))
    );
    assert!(response
        .headers_mut()
        .remove("mgmiddlewarewaittime")
        .is_some());
    assert_eq!(
        response.headers_mut().remove("strict-transport-security"),
        Some(HeaderValue::from_static(
            "max-age=31536000; includeSubDomains"
        ))
    );
    assert_eq!(
        response.headers_mut().remove("x-xss-protection"),
        Some(HeaderValue::from_static("1; mode=block"))
    );
    assert_eq!(
        response.headers_mut().remove("x-frame-options"),
        Some(HeaderValue::from_static("SAMEORIGIN"))
    );
    assert_eq!(
        response.headers_mut().remove("referrer-policy"),
        Some(HeaderValue::from_static("strict-origin"))
    );
    assert_eq!(
        response.headers_mut().remove("x-content-type-options"),
        Some(HeaderValue::from_static("nosniff"))
    );
    assert_eq!(
        response.headers_mut().remove("content-security-policy"),
        Some(HeaderValue::from_static(
            "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' \
             'unsafe-inline' 'unsafe-eval';"
        ))
    );
    let content_length = response.headers_mut().remove("content-length");
    if content_length == Some(HeaderValue::from_static("27205")) {
        // login failed
        let content = response.text().await?;
        assert!(content
            .contains("Bitte versuchen Sie es erneut. Überprüfen Sie ggf. Ihre Zugangsdaten."));
        return Err(TucanError::InvalidCredentials);
    }
    assert_eq!(content_length, Some(HeaderValue::from_static("72")));
    response.headers_mut().remove("x-powered-by"); // this header randomly appears and disappears. DO NOT ASK
    assert!(response.headers_mut().remove("date").is_some(),);
    let cookie_cnsc = response.headers_mut().remove("set-cookie").unwrap();
    let cookie_cnsc = cookie_cnsc.to_str().unwrap().trim_start_matches("cnsc =");
    let next_url_regex = Regex::new(
    r#"0; URL=/scripts/mgrqispi\.dll\?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N(?P<id>\d+),-N000019,-N000000000000000"#,
).unwrap();
    let next_url = response.headers_mut().remove("refresh").unwrap();
    let next_url = next_url.to_str().unwrap();
    let id = &next_url_regex.captures(next_url).unwrap()["id"];
    assert_eq!(response.headers().into_iter().collect::<Vec<_>>(), []);
    let content = response.text().await?;
    let _document = Html::parse_document(&content);
    Ok(LoginResponse {
        id: id.parse().unwrap(),
        cookie_cnsc: cookie_cnsc.to_owned(),
    })
}
