
#[cfg(test)]
mod tests {
    use std::{
        path,
        sync::atomic::{AtomicUsize, Ordering}, time::Duration,
    };

    use tokio::{sync::OnceCell, time::sleep};
    use webdriverbidi::{
        events::EventType, local::script::{RealmInfo, WindowRealmInfo}, remote::{
            browser::{ClientWindowNamedOrRectState, ClientWindowRectState, SetClientWindowStateParameters}, browsing_context::{BrowsingContext, CloseParameters, CreateParameters, CreateType, CssLocator, GetTree, GetTreeParameters, LocateNodesParameters, Locator, NavigateParameters, ReadinessState, SetViewportParameters, Viewport, XPathLocator}, input::{ElementOrigin, Origin, PerformActionsParameters, PointerCommonProperties, PointerMoveAction, PointerSourceAction, PointerSourceActions, SourceActions}, script::{AddPreloadScriptParameters, ChannelProperties, ChannelValue, ContextTarget, EvaluateParameters, GetRealmsParameters, RealmTarget, SharedReference, Target}, web_extension::{ExtensionData, ExtensionPath, InstallParameters}, EmptyParams
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

        let try_catch: anyhow::Result<()> = async {
            let path = std::fs::canonicalize("../../tucant-extension")?.to_str().unwrap().to_string();
            println!("{path}");
            session.web_extension_install(InstallParameters::new(ExtensionData::ExtensionPath(ExtensionPath::new(path)))).await?;

            let user_context = session.browser_create_user_context(EmptyParams::new()).await?;
            let browsing_context = session
                .browsing_context_create(CreateParameters {
                    create_type: CreateType::Window,
                    user_context: Some(user_context.user_context.clone()),
                    reference_context: None,
                    background: None,
                })
                .await?;

            session.browsing_context_set_viewport(SetViewportParameters { context: browsing_context.context.clone(), viewport: Some(Viewport { width: 1300, height: 768 }), device_pixel_ratio: None }).await?;

            /*
            let client_windows = session.browser_get_client_windows(EmptyParams::new()).await?;

            for window in client_windows.client_windows {
                session.browser_set_client_window_state(SetClientWindowStateParameters::new(window.client_window.clone(), ClientWindowNamedOrRectState::ClientWindowRectState(ClientWindowRectState { state: "normal".to_owned(), width: Some(1300), height: Some(768), x: None, y: None }))).await?;                
            }
            */

            // https://github.com/SeleniumHQ/selenium/issues/13992
            // https://github.com/w3c/webdriver-bidi/blob/main/proposals/bootstrap-scripts.md
            // https://github.com/SeleniumHQ/selenium/pull/14238/files#diff-c905a3b55dc121eee1ed81ed41659372f4e9eb47971bbdf7a876a10c44f3ff48R80

            // TODO type should be fixed in constructor
            let channel = ChannelValue::new("channel".to_owned(), ChannelProperties::new("test".to_owned(), None, None));
            session.script_add_preload_script(AddPreloadScriptParameters::new(r#"function test(channel) { channel("hi") }"#.to_owned(), Some(vec![channel]), Some(vec![browsing_context.context.clone()]), None, None)).await?;

            session.register_event_handler(EventType::ScriptMessage, async |event| {
                println!("{event:?}")
            }).await;

            navigate(&mut session, browsing_context.context.clone(), "https://www.tucan.tu-darmstadt.de/".to_owned()).await?;

            sleep(Duration::from_secs(5)).await;

            let node = session.browsing_context_locate_nodes(LocateNodesParameters::new(browsing_context.context.clone(), Locator::CssLocator(CssLocator::new("#login-username".to_owned())), None, None, None)).await?;
            panic!("{:?}", node);


            // TODO first login

            let a: Box<[PointerSourceAction]> = Box::new([
                //PointerSourceAction::PointerMoveAction(PointerMoveAction::new(0, 0, None, Some(Origin::ElementOrigin(ElementOrigin::new(SharedReference::new(shared_id, handle, extensible)))), PointerCommonProperties::new(None, None, None, None, None, None, None)))
            ]);
            let a = a.into_vec();

            let b: Box<[SourceActions]> = Box::new([
                SourceActions::PointerSourceActions(PointerSourceActions::new("1".to_owned(), None, a))
            ]);
            let b = b.into_vec();

            session.input_perform_actions(PerformActionsParameters::new(browsing_context.context.clone(), b)).await?;

/*
    let username_input = driver.query(By::Css("#login-username")).first().await?;
    let password_input = driver.find(By::Css("#login-password")).await?;
    let login_button = driver.find(By::Css("#login-button")).await?;

    let username = std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing");
    let password = std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing");
*/

            let realms = session.script_get_realms(GetRealmsParameters::new(Some(browsing_context.context.clone()), None)).await?;
            println!("{:?}", realms);
           
            let RealmInfo::WindowRealmInfo(window) = &realms.realms[0] else {
                panic!();
            };
            
            session.script_evaluate(EvaluateParameters::new("window.sayHello()".to_owned(), Target::ContextTarget(ContextTarget::new(window.context.clone(), None)), false, None, None, None)).await?;

            let contexts = session.browsing_context_get_tree(GetTreeParameters::new(None, None)).await?;
            println!("{:?}", contexts);

            // driver.query(By::XPath(r#"//div/ul/li/a[text()="Veranstaltungen"]"#)).single().await?.click().await?;

            // driver.query(By::XPath(r#"//ul/li/a[text()="Anmeldung"]"#)).single().await?.click().await?;


            sleep(Duration::from_secs(30)).await;

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
