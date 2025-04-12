
#[cfg(test)]
mod tests {
    use std::{
        path,
        sync::atomic::{AtomicUsize, Ordering}, time::Duration,
    };

    use tokio::{sync::OnceCell, time::sleep};
    use webdriverbidi::{
        local::script::{RealmInfo, WindowRealmInfo}, remote::{
            browsing_context::{BrowsingContext, CloseParameters, CreateParameters, CreateType, NavigateParameters, ReadinessState}, script::{ContextTarget, EvaluateParameters, GetRealmsParameters, RealmTarget, Target}, web_extension::{ExtensionData, ExtensionPath, InstallParameters}, EmptyParams
        }, session::WebDriverBiDiSession, webdriver::capabilities::CapabilitiesRequest
    };

    static TEST_COUNT: AtomicUsize = AtomicUsize::new(1);

    static SESSION: OnceCell<WebDriverBiDiSession> = OnceCell::const_new();

    async fn get_session() -> WebDriverBiDiSession {
        SESSION.get_or_init(async || setup_session().await.unwrap()).await.clone()
    }

    async fn setup_session() -> anyhow::Result<WebDriverBiDiSession> {
        let capabilities = CapabilitiesRequest::default();
        let mut session = WebDriverBiDiSession::new("localhost".to_owned(), 4444, capabilities);
        session.start().await?;
        Ok(session)
    }

    async fn navigate(session: &mut WebDriverBiDiSession, ctx: BrowsingContext, url: String) -> anyhow::Result<()> {
        let navigate_params = NavigateParameters::new(ctx, url, Some(ReadinessState::Complete));
        session.browsing_context_navigate(navigate_params).await?;
        Ok(())
    }

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        env_logger::init();

        // https://github.com/SeleniumHQ/selenium/issues/15585#issuecomment-2782657812
        // Firefox 138 is required
        // geckodriver --binary /home/moritz/Downloads/firefox-138.0b6/firefox/firefox-bin

        let mut session = get_session().await;
        // nix build .#extension-unpacked

        let try_catch: anyhow::Result<()> = async {
            let path = std::fs::canonicalize("../../result")?.to_str().unwrap().to_string();
            println!("{path}");
            session.web_extension_install(InstallParameters::new(ExtensionData::ExtensionPath(ExtensionPath::new(path)))).await?;

            let user_context = session.browser_create_user_context(EmptyParams::new()).await?;
            let browsing_context = session
                .browsing_context_create(CreateParameters {
                    create_type: CreateType::Window,
                    user_context: Some(user_context.user_context),
                    reference_context: None,
                    background: None,
                })
                .await?;
            navigate(&mut session, browsing_context.context.clone(), "https://www.tucan.tu-darmstadt.de/".to_owned()).await?;

            let realms = session.script_get_realms(GetRealmsParameters::new(Some(browsing_context.context.clone()), None)).await?;
            println!("{:?}", realms);
           
            let RealmInfo::WindowRealmInfo(window) = &realms.realms[0] else {
                panic!();
            };
            
            session.script_evaluate(EvaluateParameters::new("console.log(1)".to_owned(), Target::ContextTarget(ContextTarget::new(window.context.clone(), None)), false, None, None, None)).await?;


            sleep(Duration::from_secs(5)).await;

            session.browsing_context_close(CloseParameters { context: browsing_context.context, prompt_unload: None }).await?;

            Ok(())
        }.await;

        if TEST_COUNT.fetch_sub(1, Ordering::SeqCst) == 1 {
            session.close().await?;
        }

        try_catch?;

        Ok(())
    }
    /*
    #[tokio::test]
    async fn it_works2() -> anyhow::Result<()> {
        let mut session = get_session().await;
        let user_context = session.browser_create_user_context(EmptyParams::new()).await?;
        let browsing_context = session
            .browsing_context_create(CreateParameters {
                create_type: CreateType::Window,
                user_context: Some(user_context.user_context),
                reference_context: None,
                background: None,
            })
            .await?;
        navigate(&mut session, browsing_context.context.clone(), "https://google.de".to_owned()).await?;
        session.browsing_context_close(CloseParameters { context: browsing_context.context, prompt_unload: None }).await?;

        if TEST_COUNT.fetch_sub(1, Ordering::SeqCst) == 1 {
            session.close().await?;
        }

        Ok(())
    }*/
}
