use tucan_connector::{login::login, Tucan};
use tucant_types::{LoginRequest, TucanError};

#[tokio::test]
pub async fn login_incorrect() {
    let tucan = Tucan::new().await.unwrap();
    assert!(matches!(
        login(
            &tucan.client,
            &LoginRequest {
                username: "not_found".to_owned(),
                password: "not_correct".to_owned(),
            },
        )
        .await,
        Err(TucanError::InvalidCredentials)
    ))
}
