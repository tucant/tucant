pub mod html_handler;

use data_encoding::HEXLOWER;
use html_extractor::html;
use html_handler::Root;
use reqwest::{Client, ClientBuilder};
use scraper::Html;

fn main() -> Result<(), TucanError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = Tucan::new().await?;
    Ok(())
}

pub struct Tucan {
    client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum TucanError {
    #[error("HTTP error {0:?}")]
    Http(#[from] reqwest::Error),
    #[error("IO error {0:?}")]
    Io(#[from] std::io::Error),
}

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        let client = ClientBuilder::new()
            .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
            .build()?;

        /*
                let response = client
                    .get("https://www.tucan.tu-darmstadt.de/")
                    .send()
                    .await?
                    .error_for_status()?;
                println!("{response:#?}");
                let content = response.text().await?;
                let document = Html::parse_document(&content);
                println!("{}", document.html());
                let html_handler = Root::new(document.tree.root());
                let html_handler = html_handler.document_start();
                let html_handler = html_handler.doctype();
                let html_handler = html_handler.next_child_tag_open_start("html");
                let html_handler = html_handler.tag_open_end();
                html!(
                    <head>_
                        <!--"RMGklg_XASh8hhew3hZIhYXmZF9hdbOOrS4pTp7U4-Q"-->_
                        <script type="text/javascript"></script>_
                        <title>"Technische Universität Darmstadt"</title>_
                        <meta http-equiv="X-UA-Compatible" content="IE=EmulateIE9"></meta>_
                        <!--"y6RvLoAFlJ-yhWOzZ1eFLGpyCih6hv5vxd56zEkIHR4"-->_
                        <meta http-equiv="cache-control" content="no-cache"></meta>_
                        <meta http-equiv="expires" content="-1"></meta>_
                        <meta http-equiv="pragma" content="no-cache"></meta>_
                        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"></meta>_
                        <meta http-equiv="Content-Script-Type" content="text/javascript"></meta>_
                        <meta name="viewport" content="width=device-width, initial-scale=1,user-scalable=0"></meta>_
                        <link rel="shortcut icon" type="image/x-icon" href="/gfx/tuda/icons/favicon.ico"></link>_
                        <link rel="apple-touch-icon" href="/gfx/tuda/icons/iphone_touch_icon.png" type="image/gif"></link>_
                        <meta http-equiv="refresh" content="0; URL=/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001"></meta>_
                        <link href="/css/_default/dl.startpage.css" rel="stylesheet" type="text/css"></link>_
                        <script type="text/javascript" src="/js/mobile_master/jquery.js"></script>_
                        <script type="text/javascript" src="/js/mobile_master/onmediaquery.min.js"></script>_
                    </head>_
                    <body>_
                        <div id="wrapper">_
                            <a href="http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                                <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                            </a>_
                            <ul id="langMenu">_
                                <!--"OKMmJxVa9MEpv1nT-faADAcITZqNMeN44hxFZOI5duQ"-->_
                                <li><a class="img img_LangGerman" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001">"de"</a></li>_
                                <!--"B_W58bA9r6Y9MF-DHPyZIP45rNK-6Ba3bAydB3VM8DM"-->_
                                <li><a class="img img_LangEnglish" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000002">"en"</a></li>_
                            </ul>_
                        </div>_
                    </body>
                    </html>
                );

                let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001")
                    .send()
                    .await?
                    .error_for_status()?;
                println!("{response:#?}");
                let content = response.text().await?;
                let document = Html::parse_document(&content);
                println!("{}", document.html());
                let html_handler = Root::new(document.tree.root());
                let html_handler = html_handler.document_start();
                let html_handler = html_handler.doctype();
                let html_handler = html_handler.next_child_tag_open_start("html");
                let html_handler = html_handler.tag_open_end();
                html!(
                    <head>_
                        <!--"TpH4lBnEvBoB3gHo7u9UYwu2X7fAAlmIE2tkBMpvsak"-->_
                        <!--"IcATzFs-AhJLlgCbtH_f4J_riUKWfS8yoLLT9ozdTlA"-->_
                        <script type="text/javascript"></script>_
                        <title>"Technische Universität Darmstadt"</title>_
                        <meta http-equiv="X-UA-Compatible" content="IE=EmulateIE9"></meta>_
                        <meta http-equiv="cache-control" content="no-cache"></meta>_
                        <meta http-equiv="expires" content="-1"></meta>_
                        <meta http-equiv="pragma" content="no-cache"></meta>_
                        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"></meta>_
                        <meta http-equiv="Content-Script-Type" content="text/javascript"></meta>_
                        <meta name="viewport" content="width=device-width, initial-scale=1,user-scalable=0"></meta>_
                        <link href="/css/_default/dl.startpage.css" rel="stylesheet" type="text/css"></link>_
                        <link href="/css/styles.css" rel="stylesheet" type="text/css"></link>_
                        <link href="/css/colors.css" rel="stylesheet" type="text/css"></link>_
                    </head>_
                    <body class="redirect">_
                        <div id="wrapper">_
                            <a href="http://http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                                <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                            </a>_
                            <!--"MA-hDUoCrkYqlM3RsS9EUjq0y_UcuN1AB82k4O5O8YU"-->_
                            <h2><a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">"Sie werden zur Startseite weitergeleitet ..."</a></h2>_
                            <a style="text-decoration: underline;" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">"Startseite"</a>_
                        </div>_
                        <div id="sessionId" style="display: none;">"000000000000001"</div>_
                        <!--"zhJ3t6XNo2cfpZZEFiqxHJQ9udSXk5D418ej5lEytG8"-->_
                        <script>
                        "\n\t\twindow.setTimeout(function() {\n\t\t\twindow.location.href = '/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome';\n\t\t}, 500);\n\t\t"
                        </script>_
                    </body>
                    </html>
                );
        */
        // we could directly use this url but we want to be safe that we don't accidentially use wrong urls
        let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome")
        .send()
        .await?
        .error_for_status()?;
        println!("{response:#?}");
        let content = response.text().await?;
        let document = Html::parse_document(&content);
        println!("{}", document.html());
        let html_handler = Root::new(document.tree.root());
        let html_handler = html_handler.document_start();
        let html_handler = html_handler.doctype();
        html!(
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de" xmlns:msdt="uuid:C2F41010-65B3-11d1-A29F-00AA00C14882" xmlns:mso="urn:schemas-microsoft-com:office:office">
            <head>_
                <title>"Technische Universität Darmstadt"</title>_
                <!--"iPRTdQsauRZVOSpz0PmEl_ubhHccJRCaNU_bI6seaq0"-->_
                <!--"muh4fptckC_Ch7T74xLI7ivPp07sWskCVg2gy3woY28"-->_
                <meta http-equiv="X-UA-Compatible" content="IE=edge"></meta>_
                <meta http-equiv="cache-control" 		content="no-cache"></meta>_
                <meta http-equiv="expires" 				content="-1"></meta>_
                <meta http-equiv="pragma" 				content="no-cache"></meta>_
                <meta http-equiv="Content-Type" 		content="text/html; charset=utf-8"></meta>_
                <meta http-equiv="Content-Script-Type"	content="text/javascript"></meta>_
                <meta name="referrer" content="origin"></meta>_
                <meta name="keywords" content="Datenlotsen,Datenlotsen Informationssysteme GmbH,CampusNet,Campus Management"></meta>_
                <!--"PVD_IUFslfLcokMkhhqUJ2XUD8f4-KrQiSrt7qeobqU"-->_
                <link rel="shortcut icon" type="image/x-icon" href="/gfx/tuda/icons/favicon.ico"></link>_
                <script src="/js/jquery-3.6.0.min.js" 	type="text/javascript"></script>_
                <script src="/js/checkDate.js" 	type="text/javascript"></script>_
                <script src="/js/edittext.js" 	type="text/javascript"></script>_
                <script src="/js/skripts.js" 	type="text/javascript"></script>_
                <script src="/js/x.js" 			type="text/javascript"></script>_
                <!-- "-cBtAUCsH5L1QCSAXhrWUyjqREZ-qAM6anBuGb0jpis"-->_
                <link id="defLayout" 	href="/css/_default/def_layout.css"	rel="stylesheet"  type="text/css"	media="screen"></link>_
                <link id="defMenu" 		href="/css/_default/def_menu.css" 	rel="stylesheet"  type="text/css"	media="screen"></link>_
                <link id="defStyles" 	href="/css/_default/def_styles.css"	rel="stylesheet"  type="text/css"	></link><!-- "8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY" -->_
                <link id="pagePrint" 	href="/css/_default/def_print.css" 	rel="stylesheet"  type="text/css"	media="print"></link>_
                <!-- "tsCXIkgf7AHAT6f4SFdkYqr9qZ1RI2wPidDGXYoyb-M" -->_
                <link id="pageStyle"		href="/css/styles.css"	rel="stylesheet"	type="text/css"  	></link>_<!-- "8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY" -->_
                <link id="pageColors"		href="/css/colors.css"	rel="stylesheet" 	type="text/css" 	media="screen" ></link>_
                <!--"Bv96RRDpRJh6Mov4faCHyudSmfHE7HfK_7sTjNxd1wY" -->_
                <!--"dIBUikqFO2tcT78tvc7dv_E180BxF6LhwTNb4gpSuQM"-->_
                <!--"fD1xdYETGI2QrMhnwhN-3obm-UIuRhNpzKv2Qbz53Ac"-->_
                <!--"NIHfntnP_QYxOqBt0vrT3UIfpe7DzzHCCiQbHrVLrXE"-->_
                <!--"x2WUiOGjWA_UDiUqZA9skrh_uNAWGlcC-R__ip9vYyg"-->_
                <style type="text/css">
                    "oiK6m4ZNKQoGD_x_6V3-YFNSsLMUaXrX5lQwN4Q88fc"
                </style>_
            </head>_
            <body class="external_pages">_
                <div id="Cn-system-desc">_</div>_
                <script type="text/javascript">
                    "JSQjAjNPl1OG1yTeHLoTj6JEhV74LO2CassBGq9DPqo"
                </script>_
                <div id="acc_pageDescription" class="hidden"><a name="keypadDescription" class="hidden">"keypadDescription"</a>
                    "TvMjLPj4FsS4YUVJn3nppMhuQYkGn5LXsWX2f54ngjY"
                    <a href="#mainNavi" accesskey="1">"1 Hauptmenü"</a>_
                    <a href="#mainContent" accesskey="2">"2 Inhalt"</a>_
                    <a href="#keypadDescription" accesskey="3">"3 Zurück zu dieser Anleitung"</a>_
                </div>_
                <div id="pageContainer" class="pageElementTop">_
                    <div class="invAnchor">_
                        <a name="top" class="invAnchor"></a>_
                    </div>_
                    <div id="pageHead" class="pageElementTop">_
                        <div id="pageHeadTop" class="pageElementTop">_
                            <a href="?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Aimprint" class="img img_arrowImprint pageElementLeft">"Impressum"</a>_
                            <a href="?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Acontact" class="img img_arrowContact pageElementLeft">"Kontakt"</a>_
                            <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft">"Drucken"</a>_
                            <a href="#bottom" class="img img_arrowDown pageElementRight">"Zum Ende der Seite"</a>_
                        </div>_
                        <div id="pageHeadCenter" class="pageElementTop">_
                        <div id="pageHeadLeft" class="pageElementLeft">_
                            <a href="http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                                <img id="imagePageHeadLeft" src="/gfx/tuda/logo.gif" alt="Logo Technische Universität Darmstadt"></img>_
                            </a>_
                        </div>_
                        <div id="pageHeadRight" class="pageElementRight">
                            _
                        </div>_
                    </div>_
                    <div id="pageHeadBottom_1" class="pageElementTop">_
                        <div id="pageHeadControlsLeft" class="pageElementLeft">_
                            <a class="img pageHeadLink" href="#" id="extraNav_link1" target="_blank">"Homepage"</a>_
                            <a class="img pageHeadLink" href="#" id="extraNav_link2" target="_blank">"standardLink undef"</a>_
                        </div>_
                        <div id="pageHeadControlsRight" class="pageElementRight">_
                            <a class="img" href="#" id="extraNav_link3" target="_blank">"standardLink undef"</a>_
                            <a class="img" href="#" id="extraNav_link4" target="_blank">"standardLink undef"</a>_
                            <a class="img" href="#" id="extraNav_link5" target="_blank">_</a>_
                        </div>_
                    </div>_
                    <div id="pageHeadBottom_2" class="pageElementTop">_
                        <div id="pageHeadBottom_2sub_1" class="pageElementTop">_</div>_
                        <div id="pageHeadBottom_2sub_2" class="pageElementTop">_</div>_
                    </div>_
                    <div id="pageTopNavi" class="pageElementTop">_<!--"ZBEoCwQSg8mkwKf8K01M4zjrmKcSX5rBMXmyr7o7Z5M"-->_
                        <a name="mainNavi" class="hidden">_</a>_<!--"IC0hcooG1AR9WlCqc3It73C95p-H60EQzIprCbZQSoM"-->

        );

        Ok(Self { client })
    }
}
