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
                        <a name="mainNavi" class="hidden">_</a>_<!--"IC0hcooG1AR9WlCqc3It73C95p-H60EQzIprCbZQSoM"-->_
                        <ul class="nav depth_1 linkItemContainer">
                        <li class="intern depth_1 linkItem " title="Startseite" id="link000344"><a  class="depth_1 link000344 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome" >"Startseite"</a></li>
                        <li class="tree depth_1 linkItem branchLinkItem " title="Vorlesungsverzeichnis (VV)" id="link000334">
                          <a  class="depth_1 link000334 navLink branchLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AGffpSWCU7sIVWSJy1aAy1jmG3q4aceIY~XwaHIzVrLdvQPkP-lzDQQ9I-02qEpyWFwPxWu2KvTBYrd8xZxjMW2arnfw8HjKNILwdSW1BdRKtP9f8XQzmKYGi23J-ciyVwXcn6i2W1h-ZveH3jGph8bzvSoT4m2VUI5-Ib8n3mamkOhqkuRP3ifw27Q__" >"Vorlesungsverzeichnis (VV)"</a>
                          <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Lehrveranstaltungssuche" id="link000335"><a  class="depth_2 link000335 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AvshvFctL7hQdjEZStxMOoFNZCs~2AQkfpnAamtEXPlEUObyZGOKXF~pm320yxDQXuDEZ6rtqOgeA6AOK4OJJsmJ4WvPhe6DicoD5HS5msmkjgMyXVOulewKexZLRl7kzF1xgVr10BFicI3x3EnsCQG~Ijibc" >"Lehrveranstaltungssuche"</a></li>
                            <li class="intern depth_2 linkItem " title="Raumsuche" id="link000385"><a  class="depth_2 link000385 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N000000000000001,-N000385," >"Raumsuche"</a></li>
                            <li class="intern depth_2 linkItem " title="Aktuell - Wintersemester 2024/25" id="link000698"><a  class="depth_2 link000698 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AawC4udHLFvAgcWZIEUF-37qLIm5htsYcfn3GCVKQiQqmYZB7YLxVZzd4-TsYx-WNkg2jG3Fv1HbA4Iv6oFZoCSQSWimEmlInDUSZk2sxsj4mnWxvryvbUKEB2NMxT720z5ZsZR7BYA437~F~QO7wbRLGnRKH3aHzfD6m-re7Otycx99ax1jJZycOcQ__" >"Aktuell - Wintersemester 2024/25"</a></li>
                            <li class="intern depth_2 linkItem " title="Vorlesungsverzeichnis des SoSe2024" id="link000690"><a  class="depth_2 link000690 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AEWe2gy47NG56rSkCHBPp-4LxVd~JF0cLREouo5pTwiNAFpFU70ZfSQuaexRd5L87HIIeWy69jfif94hBqvFnfGNTNZK2K1dA1OmzcQIp8aDzi3JxAcel54FBHZ1gdQo1a2hzS0oZA~-hXGzVU3WeVi6FIbyY-2X0zV5LxncdRum~-InGRFU8F8-mhw__" >"Vorlesungsverzeichnis des SoSe2024"</a></li>
                            <li class="intern depth_2 linkItem " title="Vorlesungsverzeichnis des WiSe 2023/24" id="link000683"><a  class="depth_2 link000683 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Add~7axH7-kWlFTGYLiTX2eehoS0zRuiWOr7IG~9n5FlI97vOc5VaxmI5ENZrBrSDa5JgqYOEvFGqcd3pcPjtvRPxULjsMbU6XQt0PkvJXzBshpCxjRV5-NjI0Jn9YZ1mh6f~n~ZVcV7dsVEUmXsE4GV-ErL0NHptxFDvxABQD4UXZogIyT9Wmtd22g__" >"Vorlesungsverzeichnis des WiSe 2023/24"</a></li>
                            <li class="tree depth_2 linkItem branchLinkItem " title="Archiv" id="link000463">
                              <a  class="depth_2 link000463 navLink branchLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000463,-Avvarchivstart%2Ehtml" >"Archiv"</a>
                              <ul class="nav depth_3 linkItemContainer">
                                <li class="intern depth_3 linkItem " title="Sommersemster 2023" id="link000697"><a  class="depth_3 link000697 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AA4856HLHSpi8hiJEZ3iMV~xDjSbaqTo-WbSe7mhoLMouSmfsqW925rSAaVsKa2ITl0yAzpPLplsBi6aqnhfDi4TXD~FgypjbC1DRX1SUowuBuhwdl~mniKnM1rl1SNqGWv1CxrVwd49XsCOA8rqRBWIpYjWGQeAsL0lt3G93EGgAkhyWSkvqVYc5Wg__" >"Sommersemster 2023"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2022/23" id="link000689"><a  class="depth_3 link000689 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AU2x5TP~984qyD2RwvKspneoB~ieYKCRpSSFj~z-dLOUkgN~oIV0mqH7JzEBc6cz6iEq~87~iLfUds91kRdwD-k4k~4JSoFgkmPyf3mh3GN8hYSApz3zuAFRj7CEeV3yHnbyxwD~0KjQ8Nw-uz7fi2eqS8-9BYPZ9pkLWAg3h985Y6I3daZyW6jO4Tg__" >"Wintersemester 2022/23"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2022" id="link000682"><a  class="depth_3 link000682 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AJjj2PQSDS5Z0hK5IfSImEJJoOZ9MZ0MroNanpsitFlRDoufLQCdYL3X~J3TSmiZQOBzjEo7DDj~w54~7oFspA99rQzb6LZqFPWlGU7zLenu4NmkwuXZVlkxxkGB6vCZFcNq~hlGVmX82DNqBRWfjUrh3TLz1jXPUx3qBEhgiKk7lZ~jlCSc8ia~zPA__" >"Sommersemester 2022"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2021/22" id="link000670"><a  class="depth_3 link000670 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AovCR8zOObR223rq4DjabqcZ9rJ6HhJwkSEW7gOonV6iFddVBmTU6vn0C-EV-aos1L96eu1UPWxyP-cNPK7JYFCnJw2WSksGC71p5Y7XJ6tHhDsC-ENXurNjLpATLP6C4fRjkU1IiDyWF5Eg4xNSvdev~~6g7vnw7faYKN6ME~v3Qwp82UDjKBoATPQ__" >"Wintersemester 2021/22"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2021" id="link000663"><a  class="depth_3 link000663 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AfU0~ojlEY34rZBYkF5MtSHyptJkjIrhOi0dHj1CI1jsji-q4GELOh80DOlk8XUtaZdlEKOT9ThA2v-HGeRj4G4jPLHAOU1EPdjUJuS-MeqTPQQRPoLfJ2eaII08Ar5YMScnNgZew0ZORvA10o0VF1Wh-j-I3KvbkyWdnSsQFjtLa65BPpeV7qGocPA__" >"Sommersemester 2021"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2020/21" id="link000655"><a  class="depth_3 link000655 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AH2sPj1rtF-a9vIHCM4KU25oU4k3eVtJN~zHbnT02MRQ5uUsBQ8-ZK7q6xfjG~PgujOz7LOWZA4QkrbxhJd8TlvuKUI~o3~8fTFdyFprSiC~um0cDCUrnFuuFdncdzNMLAlCy4nIXEArQjiyhGV6-UoHQspHf-yy0TlO42QMtiNP5MB5qE6UXqqzTrQ__" >"Wintersemester 2020/21"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2020" id="link000635"><a  class="depth_3 link000635 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AKgnYctTIY2Li6Z8fmny4nWPk6Mu8dYJCHHc~6fbWR-UKCh7MfBUuBSMPGKBH26qlXSyWiA0PX~JlBizP-R9Gd8h8GSBWCDXK031QDeE7HUMj0csXnnMYoNOwtp99fU37ybY6PC4lqCK9xJpjpZ~9m02p2k2~7DweBgphiElUtWEws63hmVMUcQLhMw__" >"Sommersemester 2020"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2019/20" id="link000627"><a  class="depth_3 link000627 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AQy7X3OwpBzdsEfZGi5~rtttYiXknSK5reQGRtDhNZDWY0vcBZF~Ubm1gTJ~LfHrZO6UliCN3kNJiSB~LG6FDM4vu3GBQLCo4XfL7e0kYH5Gc1XVMw2MWA~b35S9XNZzyVaqVSpFbZ-tMUQqdkAlvXtyYhBAYABIKMxDNoU2tX4jr8o0fveEQFLgDpQ__" >"Wintersemester 2019/20"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2019" id="link000622"><a  class="depth_3 link000622 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AytXmlGjXwnIIJOiNedE5ZmFjqtB8PUmf~jofi2~ce9rbwuB1ihpVAQLtxEveLoUUHlpXIXKYGUTY2RvOLFb8iw6nBEbm~FKRO7YMpBH9pcbve5u81JF5oCxt42hzFTYjB6S6ZE8V4ED9pKYXU38QkUS6~SjDbRbn6LQEK9o8EDj9QlmHxQrl6DAugg__" >"Sommersemester 2019"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2018/19" id="link000614"><a  class="depth_3 link000614 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AeBWLA76K3GVfViIya8cWCpd06EG8NYOkOnjwFNt9s6CwSYVp14F1D5jLmql0cqmnLnP2hQC2XFS3Ld9Qib-Zi8npoieriZcA0QmMlO7pa82Y8lo66GlYNmmaGRxUozieP9ELBr0C5L0LKu0fnWJ8-RNoYn~b32KKf9JgiYtHp9BT85zRagFlE-EgMw__" >"Wintersemester 2018/19"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2018" id="link000605"><a  class="depth_3 link000605 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AQYcwRJBAyYOETkXZvGBTOrDBldVKMvqnfF1RRyNaLA0Z8hqy8NesjvTVJbLqwvLL2OvakKc7D2CfmP4pZM4dxzOZDPxk1gggAowGmQLB2JTSu2u9liYarDcZcqs1aYGgxjrj8dX987teNY-7ZRbhriaY4r6~ZwtnmK2oX19lpwvdjYoGdbL3EEAfng__" >"Sommersemester 2018"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 17/18" id="link000595"><a  class="depth_3 link000595 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AY7dZtdali1UT8MoK8gpetQ2c~cwPiH7OhWX0FlzcrZJN1aL9xvfh8BHtn0HZiKss0SVCCW6yCShyalJCgEcW8-4fB2vzqQy1nEyLjFnCDEjShheOMgM8J5AfU1OMGOc2IxEMPEO2diWIi7oFhi9LZJFuxIftRthR5naV22oJEdnMSfYlvDaWElL3Cg__" >"Wintersemester 17/18"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2017" id="link000583"><a  class="depth_3 link000583 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A5QDVmW~NimAmc0eFchJUW4GGlACNyOK3feayjm5V6u~NV7qtA1h7d1rECDN~J0Fpw04DG8hRGWu450oeNyDLTaelMvGmJYp~IPRr6cbOt0TJEJc4qFIHLD7KmFSR4r2hBjnqXVGGX2fz04q41T8RMyKwPNENiMyVcs8W4k9v9Mk5dAcuXpEZKbABeA__" >"Sommersemester 2017"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2016/17" id="link000576"><a  class="depth_3 link000576 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AUB3416ANafIEgR5GZZ5g4HKHGJ8vUYV9xdlx3GMUsAbKMH8OTjPF45poqpr8BuXZC69p88HsQv7Djwh0b3MHruR9Obg0SuSOW00ZSmlmC3rDYogmv5YHys7gsfSk4kA2TFkavM-TtS9NxrngFLUqnMszpa2G4fBqtvTT19GWxQXW0xnRAtN0lGLnmQ__" >"Wintersemester 2016/17"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2016" id="link000567"><a  class="depth_3 link000567 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A5qZLulKPTDvxLjfmye1m6eS7clYPDFpLvDlpZAaIn4P7wX6yvaIbaW7SME~YBc1-4N-lutm-twsI0XVHw74MSCue28ixtV7Ae8~Bw6Jaillnf1qJuhoAR~s2VJ8D4h-zY-hq3GJpfy7cPC9t7mPFEkNe7W7f8haPGC~B6f7EkpRNB8kiYlPNvHTegw__" >"Sommersemester 2016"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2015/16" id="link000560"><a  class="depth_3 link000560 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A495nrRXe4gH0h2YvTa-EzM6ep5Lr1uN3So-uq7tVpPCTw6xM~xpxfhY05AGbHlmCXXtnZoVTOBahNwaxHkJE2~orWD97TCUcJwLwtzxLehM3gO2t0VtT9ZWiRBCJ8Wht5pOgbGw8gi2jpXxVb0S~AEozoZl32DBV7Jal2EHfCLCtf4BsSxYKbpVu-w__" >"Wintersemester 2015/16"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2015" id="link000549"><a  class="depth_3 link000549 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AM3OxiphxOr4SGAh593H0298PRtu7m7LX6cMrTrUF9Zc2TCCZnxSTljnKgwhBLObn4AxRR96YAmsI8hTUTbxiKh8XlUG2GUqgGvLEfM-3Ul6d3MRJ6lwXzwbPC8--KmYk-~rZh0L8k~UNID6UCOc93k4H~2IqTLKqRp0Ua1ZmsXvDWY64tIUzl5PvwQ__" >"Sommersemester 2015"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2014/15" id="link000547"><a  class="depth_3 link000547 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Az2nwzqBpm7k8ouEV2~rUc6HxtotTEE0N5OtaoUAYhspgHf5IPzaJnfuDcO4EbK4li0yBR9cN98DDhmn7yC14iRbhmlEYeIjtAAyRCDlDK2Ev5PEOx5otG7KtqGRpT8GF-KdJzcmqGrRfke6T7mcCvSPq02jidi6fi45OipVN1Lykl-5uG-lR8~fzGQ__" >"Wintersemester 2014/15"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2014" id="link000530"><a  class="depth_3 link000530 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A1UxU8jdNihd2JOD~s6pzTZ1x8OfTmi17M1awbA5Lj25IHjDE9WkIaam4-m8yTYfm5DxD4ecHBX69XYsRbtNZzAmkVyHB~IulMMYmE8yEMBWlJvi4r6ahKVYLSi6dEx23NJn3Jr3Qk6MJ8MBLxLeelILiNcT9sa4M4ZKH5pGHXmnK7UxcNLNJ-zQA~w__" >"Sommersemester 2014"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2013/14" id="link000504"><a  class="depth_3 link000504 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AcKGMSS2-77IyNAyS65jSeWY~SwdrwTuRLVi6AjP8JfZ2di~Pi1pplMMupg~hP4Bq9Rc5Aeva0O~V6xvC75hz3CJJkzUlgEBfnoRg6Ti1hxbfzkXhzsGwmQX7vNZg7UDRpmwGsxJ~PIdk7UWmJM6rV6GpBPD2oCPmV2SlCMtfUg10cdb0DOVMJIZ4ww__" >"Wintersemester 2013/14"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2013" id="link000491"><a  class="depth_3 link000491 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Ajhm7zebU06JTs6ASx10Q1TAPNxwl0BHBIcto0zuHdEI4K~xBYvNpjlzwmJmKIRIkP8la5lWDzLnSnMuAsQbhBQLhRZCZGOugH~qpLdqjI-3q5MSBZQxwhRGU0zPjVxriEewBSlBwu1PMD6MzZCVDP9ZrFrS6F161bIXk8QgSRhiB9V4p7-hT301qSQ__" >"Sommersemester 2013"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2012/13" id="link000467"><a  class="depth_3 link000467 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Ai0T0cXktE7JwHiHBGeoRuzhqeHG5LWWfz04uQRrlrDELzmd3F9Pp2JeYFZxyej-JkMMoHwsH9kd-r87KrwEn1SSGvwQc7JOSHCRRBoAHd3nbZrBuLHiTGtzTsEX4RnQn-K6HuTv9XfWNou1S26yT92Oq9wwy4oE8C~ZBPRn5af2AGl-8RY-60bqKiA__" >"Wintersemester 2012/13"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2011/12" id="link000469"><a  class="depth_3 link000469 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AeaO~InIKIcCZdO17XYQIsmwjdf5qmZ9Y3TfLtmNd~IjqDPHdGSek6TKOpShY1fSm2cumRmX-7tVE71O916K6fgAv4vuYwAUsNf5i0e4qBUTfw4OZs0CmpBjvQYrRcmi6pN1CPnDVpLXhhwmgstfhSAk2myZhOYRJtUVlCPv-~-YgKnyoVxZMra-Gvw__" >"Wintersemester 2011/12"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2012" id="link000468"><a  class="depth_3 link000468 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A0YpNkKklOK9Ztngl-xj32zWXOfPxLkvXf0AJ7b7J-mtP58HWAsUGa0ka-bDUi5l9VBaomMOsS8iXEscx8XKJZbMviUxKN2dzrekz~pegPGxwXHA~kzHgygzVTfgIlP5xl9~izum~3l44hYBmGzJlm8h28taCSgfo~CwKfmlSIvQijsH2L2l8G8VfbQ__" >"Sommersemester 2012"</a></li>
                                <li class="intern depth_3 linkItem " title="Sommersemester 2011" id="link000470"><a  class="depth_3 link000470 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A5Ytt6D048aLKsuVcR4pijYcPPDuClAecmaOqkXaQhGJbsAgDpZ72wsJEydo00YcoMrpcpy8odPO76Owmk5rigJvNzSjquIPkCEMN6own8daCpkFIN4LoFl-ipSFsAYv7CdcZRhmDhlvihBSA1eIPl2oVdhNs04mnUDkyPs03ArQuNLiihT-hyWtLHQ__" >"Sommersemester 2011"</a></li>
                                <li class="intern depth_3 linkItem " title="Wintersemester 2010/11" id="link000471"><a  class="depth_3 link000471 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AaiAUFSSJxkfklu~35aU~aiaVoENgtTeEcKE~394-yVFFG-L7GSH2Nw0MMYfYoA0kGpDywoYhO146WmVRSYUjWyvD7xY-IejI4ZPAbKHOY8o6FjsIaRumaPkUnITQ-LdJ-2wxtT~aRqFi8MwnZjf3qQv2Xue7LV2W36Qi5uZFJA5v56Klz7-2JDEwYw__" >"Wintersemester 2010/11"</a></li>
                                <li class="intern depth_3 linkItem " title="Altsysteme" id="link000481"><a  class="depth_3 link000481 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000481,-Avvarchivaltsysteme%2Ehtml" >"Altsysteme"</a></li>
                              </ul>
                            </li>
                          </ul>
                        </li>
                        <li class="tree depth_1 linkItem branchLinkItem " title="TUCaN-Account" id="link000410">
                          <a  class="depth_1 link000410 navLink branchLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml" >"TUCaN-Account"</a>
                          <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Account anlegen" id="link000425"><a  class="depth_2 link000425 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425," >"Account anlegen"</a></li>
                            <li class="intern depth_2 linkItem " title="Passwort vergessen (nur für Bewerber/innen!)" id="link000426"><a  class="depth_2 link000426 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A" >"Passwort vergessen (nur für Bewerber/innen!)"</a></li>
                          </ul>
                        </li>
                        <li class="intern depth_1 linkItem " title="Hilfe" id="link000340"><a  class="depth_1 link000340 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml" >"Hilfe"</a></li>
                      </ul>_
                      </div>_
                      <div id="pageHeadBottom_3" class="pageElementTop">_
                            <div id="pageHeadSwitchLang" class="pageElementRight">_
                                <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CHANGELANGUAGE&ARGUMENTS=-N000000000000002,-N002" class="img img_LangEnglish pageElementLeft" title="English">"English"</a>_
                            </div>_
                            <form name="cn_loginForm" id="cn_loginForm" action="/scripts/mgrqispi.dll" method="post" class="pageElementRight">_
                                <div>_
                                    <fieldset id="fieldSet_login">_
                                        <legend>"Anmeldung"</legend>_
                                        <div class="formRow nb">_
                                            <div class="inputFieldLabel">_
                                                <label for="field_user">"TU-ID:"</label>_
                                                <input type="text" id="field_user" name="usrname"  size="15" class="login" maxlength="255" accesskey="n" autofocus=""></input>_
                                            </div>_
                                            <div class="inputFieldLabel">_
                                                <label for="field_pass">"Passwort:"</label>_
                                                <input type="password" id="field_pass" name="pass"  value="" size="15" class="login" maxlength="255" accesskey="p"></input>_
                                            </div>_
                                        </div>_
                                    </fieldset>_
                                    <input class="img img_arrowSubmit login_btn" type="submit" id="logIn_btn" 					value="Anmelden"       			onclick="return checkform('cn_loginForm','usrname:TU-ID,pass:Passwort','000000000000001');"></input>_
                                    <!--"416mrhkWvn83zXJacA3wOy6ZHvHNbAfVlkkb_PMmkEg"-->_
                                    <input name="APPNAME" 	type="hidden" value="CampusNet"></input>_
                                    <input name="PRGNAME" 	type="hidden" value="LOGINCHECK"></input>_
                                    <input name="ARGUMENTS" type="hidden" value="clino,usrname,pass,menuno,menu_type,browser,platform"></input>_
                                    <input name="clino" 		type="hidden" value="000000000000001"></input>_
                                    <input name="menuno" 		type="hidden" value="000344"></input>_
                                    <input name="menu_type" type="hidden" value="classic"></input>_
                                    <input name="browser" 	type="hidden" value=""></input>_
                                    <input name="platform" 	type="hidden" value=""></input>_
                                </div>_
                            </form>_
                        </div>_
                  </div>_
                  <div id="pageContentContainer" class="pageElementTop">_
                <!--"kZd6CmmgS-q3ZJsbi_QXJmy4uIhbl0Pt05ddWHx3vcs"-->_
                <div id="pageLeft" class="pageElementLeft">_<!-- "bhHbWVACRyHBE-MoOAfeLy6SUZbsJmGyCbT94cYBHHI" -->_
                     <div id="pageLeftTop"></div>_
                </div>_

                <div id="pageContent" class="pageElementLeft">_
                    <div id="featureBanner"></div>_
                    <a name="mainContent" class="hidden">_</a>_
                    <!-- "up1YWWVw7bFlV69jn_wheiJ5MLDQ9_KdGWCUZ5gGeuw" -->_
                    <div id="pageContentTop" class="pageElementTop">_
                    </div>_
                    <div id="contentSpacer_IE" class="pageElementTop">
                    <!-- "WVhEeLYGpyH0bXmFoofJIUMWxdfkLBe5aUmIdmUfqiM" -->_
                    <!--"CKcFISCJjRLw3ii080mSqvobpMA3Z3OFHiqwurhqzcI"-->_
                    <!--"Ur30ahmaXh5XzV5xIHsTj20h-0qX1_GS1SR0QttvqB0"-->_
        );

        Ok(Self { client })
    }
}
