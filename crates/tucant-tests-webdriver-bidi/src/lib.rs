#[cfg(test)]
mod tests {
    use webdriverbidi::{
        remote::{
            EmptyParams,
            browsing_context::{CreateParameters, CreateType},
        },
        session::WebDriverBiDiSession,
        webdriver::capabilities::CapabilitiesRequest,
    };

    async fn setup_session() -> Result<WebDriverBiDiSession, Box<dyn std::error::Error>> {
        let capabilities = CapabilitiesRequest::default();
        let mut session = WebDriverBiDiSession::new("localhost".to_owned(), 4444, capabilities);
        session.start().await?;
        Ok(session)
    }

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut session = setup_session().await?;
        let user_context = session.browser_create_user_context(EmptyParams::new()).await?;
        let browsing_context = session
            .browsing_context_create(CreateParameters {
                create_type: CreateType::Window,
                user_context: Some(user_context.user_context),
                reference_context: None,
                background: None,
            })
            .await?;

        Ok(())
    }
}
