#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    use serde_json::json;
    use tokio::{sync::OnceCell, time::sleep};
    use webdriverbidi::{
        events::EventType,
        model::{
            browsing_context::{BrowsingContext, CloseParameters, CssLocator, GetTreeParameters, LocateNodesParameters, Locator, NavigateParameters, ReadinessState, SetViewportParameters, Viewport},
            common::Extensible,
            input::{ElementOrigin, KeyDownAction, KeySourceAction, KeySourceActions, KeyUpAction, Origin, PerformActionsParameters, PointerCommonProperties, PointerDownAction, PointerMoveAction, PointerParameters, PointerSourceAction, PointerSourceActions, PointerType, PointerUpAction, SourceActions},
            script::{ContextTarget, EvaluateParameters, GetRealmsParameters, NodeRemoteValue, RealmInfo, SharedReference, Target},
            session::SubscriptionRequest,
            web_extension::{ExtensionData, ExtensionPath, InstallParameters},
        },
        session::WebDriverBiDiSession,
        webdriver::capabilities::CapabilitiesRequest,
    };

    static TEST_COUNT: AtomicUsize = AtomicUsize::new(1);

    static SESSION: OnceCell<WebDriverBiDiSession> = OnceCell::const_new();

    static ACTION_ID: AtomicUsize = AtomicUsize::new(1);

    async fn get_session() -> WebDriverBiDiSession {
        SESSION.get_or_init(async || setup_session().await.unwrap()).await.clone()
    }

    async fn setup_session() -> anyhow::Result<WebDriverBiDiSession> {
        let mut capabilities = CapabilitiesRequest::default();
        capabilities.add_first_match(HashMap::from([
            ("browserName".to_owned(), json!("chrome")),
            (
                "goog:chromeOptions".to_owned(),
                json!({
                    "binary": "/home/moritz/Downloads/chrome-linux64/chrome-linux64/chrome",
                    "args": ["--enable-unsafe-extension-debugging", "--remote-debugging-pipe"],
                }),
            ),
        ]));
        capabilities.add_first_match(HashMap::from([("browserName".to_owned(), json!("firefox"))]));
        let mut session = WebDriverBiDiSession::new("localhost".to_owned(), 4444, capabilities);
        session.start().await?;
        Ok(session)
    }

    async fn navigate(session: &mut WebDriverBiDiSession, ctx: BrowsingContext, url: String) -> anyhow::Result<()> {
        let navigate_params = NavigateParameters::new(ctx, url, Some(ReadinessState::Complete));
        session.browsing_context_navigate(navigate_params).await?;
        Ok(())
    }

    fn generate_keypresses(input: &str) -> Vec<KeySourceAction> {
        input.chars().flat_map(|c| [KeySourceAction::KeyDownAction(KeyDownAction::new(c.to_string())), KeySourceAction::KeyUpAction(KeyUpAction::new(c.to_string()))]).collect()
    }

    async fn click_element(session: &mut WebDriverBiDiSession, browsing_context: String, node: &NodeRemoteValue) -> anyhow::Result<()> {
        let a: Box<[PointerSourceAction]> = Box::new([
            PointerSourceAction::PointerMoveAction(PointerMoveAction::new(5.0, 5.0, None, Some(Origin::ElementOrigin(ElementOrigin::new(SharedReference { shared_id: node.shared_id.clone().unwrap(), handle: node.handle.clone(), extensible: Extensible::new() }))), PointerCommonProperties::new(None, None, None, None, None, None, None))),
            PointerSourceAction::PointerDownAction(PointerDownAction::new(0, PointerCommonProperties::new(None, None, None, None, None, None, None))),
            PointerSourceAction::PointerUpAction(PointerUpAction::new(0)),
        ]);
        let a = a.into_vec();

        let id = ACTION_ID.fetch_add(1, Ordering::Relaxed);
        let b: Box<[SourceActions]> = Box::new([SourceActions::PointerSourceActions(PointerSourceActions::new(id.to_string(), Some(PointerParameters::new(Some(PointerType::Mouse))), a))]);
        let b = b.into_vec();

        session.input_perform_actions(PerformActionsParameters::new(browsing_context.clone(), b)).await?;
        Ok(())
    }

    async fn write_text(session: &mut WebDriverBiDiSession, browsing_context: String, element: &str, input: &str) -> anyhow::Result<()> {
        let node = session.browsing_context_locate_nodes(LocateNodesParameters::new(browsing_context.clone(), Locator::CssLocator(CssLocator::new(element.to_owned())), None, None, None)).await?;
        let node = &node.nodes[0];

        click_element(session, browsing_context.clone(), node).await?;

        let id = ACTION_ID.fetch_add(1, Ordering::Relaxed);
        let e: Box<[SourceActions]> = Box::new([SourceActions::KeySourceActions(KeySourceActions::new(id.to_string(), generate_keypresses(input)))]);
        let e = e.into_vec();

        session.input_perform_actions(PerformActionsParameters::new(browsing_context.clone(), e)).await?;

        Ok(())
    }

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        dotenvy::dotenv().unwrap();
        let username = std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing");
        let password = std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing");

        env_logger::init();

        // https://github.com/SeleniumHQ/selenium/issues/15585#issuecomment-2782657812
        // Firefox 138 is required
        // geckodriver --binary /home/moritz/Downloads/firefox-138.0b6/firefox/firefox-bin

        // Download beta (>= 136.0.7103.25) chrome and chromedriver from https://googlechromelabs.github.io/chrome-for-testing/#beta
        // /home/moritz/Downloads/chromedriver-linux64/chromedriver-linux64/chromedriver --port=4444 --enable-chrome-logs
        // https://github.com/GoogleChromeLabs/chromium-bidi/issues/2849

        // https://groups.google.com/a/chromium.org/g/chromium-extensions/c/aEHdhDZ-V0E/m/WvvehPqKAwAJ
        // seems like context menu clicking etc may happen at some point in webdriver bidi

        let mut session = get_session().await;

        let try_catch: anyhow::Result<()> = async {
            let path = std::fs::canonicalize("../../tucant-extension")?.to_str().unwrap().to_string();
            println!("{path}");
            session.web_extension_install(InstallParameters::new(ExtensionData::ExtensionPath(ExtensionPath::new(path)))).await?;
            sleep(Duration::from_secs(1)).await; // wait for extension to be installed

            let contexts = session.browsing_context_get_tree(GetTreeParameters { max_depth: None, root: None }).await?;

            let browsing_context = contexts.contexts[0].context.clone().clone();

            session
                .register_event_handler(EventType::LogEntryAdded, async |event| {
                    println!("{event}");
                })
                .await;

            session.session_subscribe(SubscriptionRequest::new(vec!["log.entryAdded".to_owned()], Some(vec![browsing_context.clone()]), None)).await?;

            session
                .browsing_context_set_viewport(SetViewportParameters {
                    user_contexts: None,
                    context: Some(browsing_context.clone()),
                    viewport: Some(Viewport { width: 1300, height: 768 }),
                    device_pixel_ratio: None,
                })
                .await?;

            navigate(&mut session, browsing_context.clone(), "https://www.tucan.tu-darmstadt.de/".to_owned()).await?;

            sleep(Duration::from_secs(1)).await; // wait for frontend javascript to be executed

            write_text(&mut session, browsing_context.clone(), "#login-username", &username).await?;
            write_text(&mut session, browsing_context.clone(), "#login-password", &password).await?;

            let node = session.browsing_context_locate_nodes(LocateNodesParameters::new(browsing_context.clone(), Locator::CssLocator(CssLocator::new("#login-button".to_owned())), None, None, None)).await?;
            let node = &node.nodes[0];
            click_element(&mut session, browsing_context.clone(), node).await?;

            session
                .script_evaluate(EvaluateParameters::new(
                    r##"
                    new Promise((resolve) => {
                        const observer = new MutationObserver((mutations, observer) => {
                            const element = document.querySelector("#logout-button");
                            if (element) {
                                observer.disconnect();
                                resolve(element);
                            }
                        });

                        observer.observe(document.body, {
                            childList: true,
                            subtree: true,
                        });
                    })
                    "##
                    .to_owned(),
                    Target::ContextTarget(ContextTarget::new(browsing_context.clone(), None)),
                    true,
                    None,
                    None,
                    Some(true),
                ))
                .await?;

            let realms = session.script_get_realms(GetRealmsParameters::new(Some(browsing_context.clone()), None)).await?;
            println!("{realms:?}");

            let RealmInfo::WindowRealmInfo(window) = &realms.realms[0] else {
                panic!();
            };

            session.script_evaluate(EvaluateParameters::new(r#"chrome.runtime.sendMessage("open-in-tucan-page")"#.to_owned(), Target::ContextTarget(ContextTarget::new(browsing_context.clone(), None)), false, None, None, Some(true))).await?;

            sleep(Duration::from_secs(50)).await;

            let realms = session.script_get_realms(GetRealmsParameters::new(Some(browsing_context.clone()), None)).await?;
            println!("{realms:?}");

            let contexts = session.browsing_context_get_tree(GetTreeParameters { max_depth: None, root: Some(browsing_context.clone()) }).await?;
            println!("{contexts:?}");

            session.script_evaluate(EvaluateParameters::new(r#"window.dispatchEvent(new CustomEvent('tucant', { detail: "open-in-tucan-page" }));"#.to_owned(), Target::ContextTarget(ContextTarget::new(browsing_context.clone(), None)), false, None, None, Some(true))).await?;

            sleep(Duration::from_secs(50)).await;

            // driver.query(By::XPath(r#"//div/ul/li/a[text()="Veranstaltungen"]"#)).single().await?.click().await?;

            // driver.query(By::XPath(r#"//ul/li/a[text()="Anmeldung"]"#)).single().await?.click().await?;

            session.browsing_context_close(CloseParameters { context: browsing_context, prompt_unload: None }).await?;

            Ok(())
        }
        .await;

        if TEST_COUNT.fetch_sub(1, Ordering::SeqCst) == 1 {
            session.close().await?;
        }

        try_catch?;

        Ok(())
    }
}
