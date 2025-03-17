use tucant_types::{LoggedInHead, TucanError, VorlesungsverzeichnisUrls};

use html_handler::{InElement, InRoot, Root};

#[must_use]
pub fn html_head_2<'a>(html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>) -> InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>> {
    html_extractor::html! {
        <!--"TpH4lBnEvBoB3gHo7u9UYwu2X7fAAlmIE2tkBMpvsak"-->_
        <!--"IcATzFs-AhJLlgCbtH_f4J_riUKWfS8yoLLT9ozdTlA"-->_
        <script type="text/javascript">
        </script>_
        <title>
            "Technische Universität Darmstadt"
        </title>_
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
    };
    html_handler
}

#[must_use]
pub fn html_head<'a>(html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>) -> Result<InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>, TucanError> {
    html_extractor::html! {
        <title>
            "Technische Universität Darmstadt"
        </title>_
        <!--"iPRTdQsauRZVOSpz0PmEl_ubhHccJRCaNU_bI6seaq0"-->_
        <!--"muh4fptckC_Ch7T74xLI7ivPp07sWskCVg2gy3woY28"-->_
        <meta http-equiv="X-UA-Compatible" content="IE=edge"></meta>_
        <meta http-equiv="cache-control" content="no-cache"></meta>_
        <meta http-equiv="expires" content="-1"></meta>_
        <meta http-equiv="pragma" content="no-cache"></meta>_
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"></meta>_
        <meta http-equiv="Content-Script-Type" content="text/javascript"></meta>_
        <meta name="referrer" content="origin"></meta>_
        <meta name="keywords" content="Datenlotsen,Datenlotsen Informationssysteme GmbH,CampusNet,Campus Management"></meta>_
        <!--"PVD_IUFslfLcokMkhhqUJ2XUD8f4-KrQiSrt7qeobqU"-->_
        <link rel="shortcut icon" type="image/x-icon" href="/gfx/tuda/icons/favicon.ico"></link>_
        <script src="/js/jquery-3.6.0.min.js" type="text/javascript">
        </script>_
        <script src="/js/checkDate.js" type="text/javascript">
        </script>_
        <script src="/js/edittext.js" type="text/javascript">
        </script>_
        <script src="/js/skripts.js" type="text/javascript">
        </script>_
        <script src="/js/x.js" type="text/javascript">
        </script>_
        <!--"-cBtAUCsH5L1QCSAXhrWUyjqREZ-qAM6anBuGb0jpis"-->_
        <link id="defLayout" href="/css/_default/def_layout.css" rel="stylesheet" type="text/css" media="screen"></link>_
        <link id="defMenu" href="/css/_default/def_menu.css" rel="stylesheet" type="text/css" media="screen"></link>_
        <link id="defStyles" href="/css/_default/def_styles.css" rel="stylesheet" type="text/css"></link>
        <!--"8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY"-->_
        <link id="pagePrint" href="/css/_default/def_print.css" rel="stylesheet" type="text/css" media="print"></link>_
        <!--"tsCXIkgf7AHAT6f4SFdkYqr9qZ1RI2wPidDGXYoyb-M"-->_
        <link id="pageStyle" href="/css/styles.css" rel="stylesheet" type="text/css"></link>_
        <!--"8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY"-->_
        <link id="pageColors" href="/css/colors.css" rel="stylesheet" type="text/css" media="screen"></link>_
        <!--"Bv96RRDpRJh6Mov4faCHyudSmfHE7HfK_7sTjNxd1wY"-->_
        <!--"dIBUikqFO2tcT78tvc7dv_E180BxF6LhwTNb4gpSuQM"-->_
        <!--"fD1xdYETGI2QrMhnwhN-3obm-UIuRhNpzKv2Qbz53Ac"-->_
        <!--"NIHfntnP_QYxOqBt0vrT3UIfpe7DzzHCCiQbHrVLrXE"-->_
        <!--"x2WUiOGjWA_UDiUqZA9skrh_uNAWGlcC-R__ip9vYyg"-->_
    }
    if html_handler.peek().is_none() {
        html_extractor::html! {
            </head>_
        }
        if html_handler.peek().unwrap().value().as_element().unwrap().has_class("timeout", scraper::CaseSensitivity::CaseSensitive) {
            html_extractor::html! {
                <body class="timeout">
            };
            let _html_handler = html_handler;
            return Err(TucanError::Timeout);
        } else if html_handler.peek().unwrap().value().as_element().unwrap().has_class("access_denied", scraper::CaseSensitivity::CaseSensitive) {
            html_extractor::html! {
                <body class="access_denied">
            };
            let _html_handler = html_handler;
            return Err(TucanError::AccessDenied);
        } else {
            panic!();
        }
    }
    Ok(html_handler)
}

#[must_use]
pub fn page_start<'a>(html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>) -> InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>>>> {
    html_extractor::html! {
        <div id="Cn-system-desc">_
        </div>_
        <script type="text/javascript">
            "JSQjAjNPl1OG1yTeHLoTj6JEhV74LO2CassBGq9DPqo"
        </script>_
        <div id="acc_pageDescription" class="hidden">
            <a name="keypadDescription" class="hidden">
                "keypadDescription"
            </a>
            "TvMjLPj4FsS4YUVJn3nppMhuQYkGn5LXsWX2f54ngjY"
            <a href="#mainNavi" accesskey="1">
                "1 Hauptmenü"
            </a>_
            <a href="#mainContent" accesskey="2">
                "2 Inhalt"
            </a>_
            <a href="#keypadDescription" accesskey="3">
                "3 Zurück zu dieser Anleitung"
            </a>_
        </div>_
        <div id="pageContainer" class="pageElementTop">_
            <div class="invAnchor">_
                <a name="top" class="invAnchor">
                </a>_
            </div>_
            <div id="pageHead" class="pageElementTop">_
                <div id="pageHeadTop" class="pageElementTop">_
                    <a href=_imprint_url class="img img_arrowImprint pageElementLeft">
                        "Impressum"
                    </a>_
                    <a href=_contact_url class="img img_arrowContact pageElementLeft">
                        "Kontakt"
                    </a>_
                    <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft">
                        "Drucken"
                    </a>_
                    <a href="#bottom" class="img img_arrowDown pageElementRight">
                        "Zum Ende der Seite"
                    </a>_
                </div>_
                <div id="pageHeadCenter" class="pageElementTop">_
                    <div id="pageHeadLeft" class="pageElementLeft">_
                        <a href="http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                            <img id="imagePageHeadLeft" src="/gfx/tuda/logo.gif" alt="Logo Technische Universität Darmstadt"></img>_
                        </a>_
                    </div>_
                    <div id="pageHeadRight" class="pageElementRight">_
                    </div>_
                </div>_
                <div id="pageHeadBottom_1" class="pageElementTop">_
                    <div id="pageHeadControlsLeft" class="pageElementLeft">_
                        <a class="img pageHeadLink" href="#" id="extraNav_link1" target="_blank">
                            "Homepage"
                        </a>_
                        <a class="img pageHeadLink" href="#" id="extraNav_link2" target="_blank">
                            "standardLink undef"
                        </a>_
                    </div>_
                    <div id="pageHeadControlsRight" class="pageElementRight">_
                        <a class="img" href="#" id="extraNav_link3" target="_blank">
                            "standardLink undef"
                        </a>_
                        <a class="img" href="#" id="extraNav_link4" target="_blank">
                            "standardLink undef"
                        </a>_
                        <a class="img" href="#" id="extraNav_link5" target="_blank">_
                        </a>_
                    </div>_
                </div>_
                <div id="pageHeadBottom_2" class="pageElementTop">_
                    <div id="pageHeadBottom_2sub_1" class="pageElementTop">_
                    </div>_
                    <div id="pageHeadBottom_2sub_2" class="pageElementTop">_
                    </div>_
                </div>_
                <div id="pageTopNavi" class="pageElementTop">_
                    <!--"ZBEoCwQSg8mkwKf8K01M4zjrmKcSX5rBMXmyr7o7Z5M"-->_
                    <a name="mainNavi" class="hidden">_
                    </a>_
                    <!--"IC0hcooG1AR9WlCqc3It73C95p-H60EQzIprCbZQSoM"-->_
                    <ul class="nav depth_1 linkItemContainer">
    };
    html_handler
}

#[must_use]
pub fn vv_something<'a>(html_handler: InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>>>>>, id: u64) -> (InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>>>>>, VorlesungsverzeichnisUrls) {
    let mut vvs = Vec::new();
    // these link ids are incrementing so they are different if used from different contexts. could in theory be calculated based on some starting number
    html_extractor::html! {
        <ul class="nav depth_2 linkItemContainer">
            <li class="intern depth_2 linkItem " title="Lehrveranstaltungssuche" id=_id>
                <a class=_class href=lehrveranstaltungssuche_url>
                    "Lehrveranstaltungssuche"
                </a>
            </li>
            <li class="intern depth_2 linkItem " title="Raumsuche" id=_id>
                <a class=_class href=_raumsuche_url>
                    "Raumsuche"
                </a>
            </li>
            <li class="intern depth_2 linkItem " title=_aktuell_title id=_id>
                <a class=_class href=aktuell_url>
                    aktuell_title
                </a>
            </li>
            <li class="intern depth_2 linkItem " title=_title_wise202425 id=_linkclass>
                <a class=_linkclass href=vv_1_url>
                    vv_1_title
                </a>
            </li>
            <li class="intern depth_2 linkItem " title=_title_wise202425 id=_linkclass>
                <a class=_linkclass href=vv_2_url>
                    vv_2_title
                </a>
            </li>
            let a = if id != 1 {
                <li class="intern depth_2 linkItem " title=_title_wise202421 id=_linkclass>
                    <a class=_linkclass href=vv_3_url>
                        vv_3_title
                    </a>
                </li>
            } => vvs.push((vv_3_title, vv_3_url));
            <li class="tree depth_2 linkItem branchLinkItem " title="Archiv" id=_linkclass>
                <a class=_linkclass href=_url>
                    "Archiv"
                </a>
                <ul class="nav depth_3 linkItemContainer">
                    let temp_vec = while html_handler.peek().is_some() {
                        <li class="intern depth_3 linkItem " title=_title id=_linkclass>
                            <a class=_linkclass href=_url>
                                _text
                            </a>
                        </li>
                    } => ();
                </ul>
            </li>
        </ul>
    };
    vvs.insert(0, (aktuell_title, aktuell_url));
    vvs.insert(1, (vv_1_title, vv_1_url));
    vvs.insert(2, (vv_2_title, vv_2_url));
    (html_handler, VorlesungsverzeichnisUrls { lehrveranstaltungssuche_url, vvs })
}

#[must_use]
pub fn logged_in_head<'a>(html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>, id: u64) -> (InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>>>>, LoggedInHead) {
    assert_ne!(id, 1);
    html_extractor::html! {
        use page_start(html_handler);
        <li class="tree depth_1 linkItem branchLinkItem " title="Aktuelles" id="link000019">
            <a class="depth_1 link000019 navLink branchLink " href=_aktuelles_url>
                "Aktuelles"
            </a>
            <ul class="nav depth_2 linkItemContainer">
                <li class="intern depth_2 linkItem " title="Nachrichten" id="link000299">
                    <a class="depth_2 link000299 navLink " href=messages_url>
                        "Nachrichten"
                    </a>
                </li>
            </ul>
        </li>
        <li class="tree depth_1 linkItem branchLinkItem " title="VV" id="link000326">
            <a class="depth_1 link000326 navLink branchLink " href=vorlesungsverzeichnis_url>
                "VV"
            </a>
            let vv = vv_something(html_handler, id);
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="Stundenplan" id="link000268">
                        <a class="depth_1 link000268 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000268,-A,-A,-N1")}>
                            "Stundenplan"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Tagesansicht" id="link000269">
                                <a class="depth_2 link000269 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000269,-A,-A,-N0")}>
                                    "Tagesansicht"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Wochenansicht" id="link000270">
                                <a class="depth_2 link000270 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000270,-A,-A,-N1")}>
                                    "Wochenansicht"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Monatsansicht" id="link000271">
                                <a class="depth_2 link000271 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{id:015},-N000271,-A")}>
                                    "Monatsansicht"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Export" id="link000272">
                                <a class="depth_2 link000272 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER_EXPORT&ARGUMENTS=-N{id:015},-N000272,")}>
                                    "Export"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="Veranstaltungen" id="link000273">
                        <a class="depth_1 link000273 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000273,-Astudveranst%2Ehtml")}>
                            "Veranstaltungen"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Meine Module" id="link000275">
                                <a class="depth_2 link000275 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N{id:015},-N000275,")}>
                                    "Meine Module"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Meine Veranstaltungen" id="link000274">
                                <a class="depth_2 link000274 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N{id:015},-N000274,")}>
                                    "Meine Veranstaltungen"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Meine Wahlbereiche" id="link000307">
                                <a class="depth_2 link000307 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N{id:015},-N000307,")}>
                                    "Meine Wahlbereiche"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Anmeldung" id="link000311">
                                <a class="depth_2 link000311 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{id:015},-N000311,-A")}>
                                    "Anmeldung"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Mein aktueller Anmeldestatus" id="link000308">
                                <a class="depth_2 link000308 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYREGISTRATIONS&ARGUMENTS=-N{id:015},-N000308,-N000000000000000")}>
                                    "Mein aktueller Anmeldestatus"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="Prüfungen" id="link000280">
                        <a class="depth_1 link000280 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000280,-Astudpruefungen%2Ehtml")}>
                            "Prüfungen"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Meine Prüfungen" id="link000318">
                                <a class="depth_2 link000318 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N{id:015},-N000318,")}>
                                    "Meine Prüfungen"
                                </a>
                            </li>
                            <li class="tree depth_2 linkItem branchLinkItem " title="Mein Prüfungsplan" id="link000389">
                                <a class="depth_2 link000389 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCPCHOICE&ARGUMENTS=-N{id:015},-N000389,")}>
                                    "Mein Prüfungsplan"
                                </a>
                                <ul class="nav depth_3 linkItemContainer">
                                    <li class="intern depth_3 linkItem " title="Wichtige Hinweise" id="link000391">
                                        <a class="depth_3 link000391 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000391,-Astudplan%2Ehtml")}>
                                            "Wichtige Hinweise"
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="tree depth_2 linkItem branchLinkItem " title="Semesterergebnisse" id="link000323">
                                <a class="depth_2 link000323 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000323,-Astudergebnis%2Ehtml")}>
                                    "Semesterergebnisse"
                                </a>
                                <ul class="nav depth_3 linkItemContainer">
                                    <li class="intern depth_3 linkItem " title="Modulergebnisse" id="link000324">
                                        <a class="depth_3 link000324 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N{id:015},-N000324,")}>
                                            "Modulergebnisse"
                                        </a>
                                    </li>
                                    <li class="intern depth_3 linkItem " title="Prüfungsergebnisse" id="link000325">
                                        <a class="depth_3 link000325 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N{id:015},-N000325,")}>
                                            "Prüfungsergebnisse"
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="intern depth_2 linkItem " title="Leistungsspiegel" id="link000316">
                                <a class="depth_2 link000316 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N{id:015},-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000")}>
                                    "Leistungsspiegel"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="Service" id="link000337">
                        <a class="depth_1 link000337 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000337,-Aservice%2Ehtml")}>
                            "Service"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Persönliche Daten" id="link000339">
                                <a class="depth_2 link000339 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PERSADDRESS&ARGUMENTS=-N{id:015},-N000339,-A")}>
                                    "Persönliche Daten"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Meine Dokumente" id="link000557">
                                <a class="depth_2 link000557 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{id:015},-N000557,")}>
                                    "Meine Dokumente"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Anträge" id="link000600">
                                <a class="depth_2 link000600 navLink " href=antraege_url>
                                    "Anträge"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Sperren" id="link000652">
                                <a class="depth_2 link000652 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=HOLDINFO&ARGUMENTS=-N{id:015},-N000652,")}>
                                    "Sperren"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="Bewerbung" id="link000441">
                        <a class="depth_1 link000441 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000441,-Abewerbung")}>
                            "Bewerbung"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Herzlich Willkommen" id="link000442">
                                <a class="depth_2 link000442 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000442,-Abewerbung")}>
                                    "Herzlich Willkommen"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Meine Bewerbung" id="link000443">
                                <a class="depth_2 link000443 navLink " href=meine_bewerbung_url>
                                    "Meine Bewerbung"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Meine Dokumente" id="link000444">
                                <a class="depth_2 link000444 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{id:015},-N000444,")}>
                                    "Meine Dokumente"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="intern depth_1 linkItem " title="Hilfe" id="link000340">
                        <a class="depth_1 link000340 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000340,-Ahilfe%2Ehtml")}>
                            "Hilfe"
                        </a>
                    </li>
                </ul>_
            </div>_
            <div id="pageHeadBottom_3" class="pageElementTop">_
                <div id="pageHeadSwitchLang" class="pageElementRight">_
                    <a href=_wef class="img img_LangEnglish pageElementLeft" title="English">
                        "English"
                    </a>_
                    <a href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOGOUT&ARGUMENTS=-N{id:015},-N001")} id="logoutButton" class="img img_arrowLogout logout" title="Abmelden">
                        "Abmelden"
                    </a>_
                </div>_
            </div>_
        </div>_
        <div id="pageContentContainer" class="pageElementTop">_
            <!--"kZd6CmmgS-q3ZJsbi_QXJmy4uIhbl0Pt05ddWHx3vcs"-->_
            <div id="pageLeft" class="pageElementLeft">_
                <!--"bhHbWVACRyHBE-MoOAfeLy6SUZbsJmGyCbT94cYBHHI"-->_
                <div id="pageLeftTop">
                </div>_
            </div>_
            <div id="pageContent" class="pageElementLeft">_
                <div id="featureBanner">
                </div>_
                <a name="mainContent" class="hidden">_
                </a>_
                <!--"up1YWWVw7bFlV69jn_wheiJ5MLDQ9_KdGWCUZ5gGeuw"-->_
                <div id="pageContentTop" class="pageElementTop">_
                    <div id="loginData">_
                        <span class="loginDataLoggedAs">
                            <b>
                                "Sie sind angemeldet als"
                                <span class="colon">
                                    ":"
                                </span>_
                            </b>
                        </span>_
                        <span class="loginDataName" id="loginDataName">
                            <b>
                                "Name"
                                <span class="colon">
                                    ":"
                                </span>_
                            </b>
                            _full_name
                        </span>_
                        <span class="loginDataDate">
                            <b>
                                "am"
                                <span class="colon">
                                    ":"
                                </span>
                            </b>
                            _date
                        </span>_
                        <span class="loginDataTime">
                            <b>
                                "um"
                                <span class="colon time_colon">
                                    ":"
                                </span>
                            </b>
                            _time
                        </span>_
                    </div>_
                </div>_
                <div id="contentSpacer_IE" class="pageElementTop">
                    <!--"WVhEeLYGpyH0bXmFoofJIUMWxdfkLBe5aUmIdmUfqiM"-->_
                    <!--"CKcFISCJjRLw3ii080mSqvobpMA3Z3OFHiqwurhqzcI"-->_
    };
    (html_handler, LoggedInHead { messages_url, vorlesungsverzeichnis_url, vv, antraege_url, meine_bewerbung_url })
}

#[must_use]
pub fn logged_out_head<'a>(html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>, menuno: u64) -> InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>>>> {
    html_extractor::html! {
        use page_start(html_handler);
        <li class="intern depth_1 linkItem " title="Startseite" id="link000344">
            <a class="depth_1 link000344 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">
                "Startseite"
            </a>
        </li>
        <li class="tree depth_1 linkItem branchLinkItem " title="Vorlesungsverzeichnis (VV)" id="link000334">
            <a class="depth_1 link000334 navLink branchLink " href=_url>
                "Vorlesungsverzeichnis (VV)"
            </a>
            let _unused = vv_something(html_handler, 1);
                    </li>
                    <li class="tree depth_1 linkItem branchLinkItem " title="TUCaN-Account" id="link000410">
                        <a class="depth_1 link000410 navLink branchLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml">
                            "TUCaN-Account"
                        </a>
                        <ul class="nav depth_2 linkItemContainer">
                            <li class="intern depth_2 linkItem " title="Account anlegen" id="link000425">
                                <a class="depth_2 link000425 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425,">
                                    "Account anlegen"
                                </a>
                            </li>
                            <li class="intern depth_2 linkItem " title="Passwort vergessen (nur für Bewerber/innen!)" id="link000426">
                                <a class="depth_2 link000426 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A">
                                    "Passwort vergessen (nur für Bewerber/innen!)"
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="intern depth_1 linkItem " title="Hilfe" id="link000340">
                        <a class="depth_1 link000340 navLink " href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml">
                            "Hilfe"
                        </a>
                    </li>
                </ul>_
            </div>_
            <div id="pageHeadBottom_3" class="pageElementTop">_
                <div id="pageHeadSwitchLang" class="pageElementRight">_
                    <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CHANGELANGUAGE&ARGUMENTS=-N000000000000002,-N002" class="img img_LangEnglish pageElementLeft" title="English">
                        "English"
                    </a>_
                </div>_
                <form name="cn_loginForm" id="cn_loginForm" action="/scripts/mgrqispi.dll" method="post" class="pageElementRight">_
                    <div>_
                        <fieldset id="fieldSet_login">_
                            <legend>
                                "Anmeldung"
                            </legend>_
                            <div class="formRow nb">_
                                <div class="inputFieldLabel">_
                                    <label for="field_user">
                                        "TU-ID:"
                                    </label>_
                                    <input type="text" id="field_user" name="usrname" size="15" class="login" maxlength="255" accesskey="n" autofocus=""></input>_
                                </div>_
                                <div class="inputFieldLabel">_
                                    <label for="field_pass">
                                        "Passwort:"
                                    </label>_
                                    <input type="password" id="field_pass" name="pass" value="" size="15" class="login" maxlength="255" accesskey="p"></input>_
                                </div>_
                            </div>_
                        </fieldset>_
                        <input class="img img_arrowSubmit login_btn" type="submit" id="logIn_btn" value="Anmelden" onclick="return checkform('cn_loginForm','usrname:TU-ID,pass:Passwort','000000000000001');"></input>_
                        <!--"416mrhkWvn83zXJacA3wOy6ZHvHNbAfVlkkb_PMmkEg"-->_
                        <input name="APPNAME" type="hidden" value="CampusNet"></input>_
                        <input name="PRGNAME" type="hidden" value="LOGINCHECK"></input>_
                        <input name="ARGUMENTS" type="hidden" value="clino,usrname,pass,menuno,menu_type,browser,platform"></input>_
                        <input name="clino" type="hidden" value="000000000000001"></input>_
                        <input name="menuno" type="hidden" value={&format!("{menuno:0>6}")}></input>_
                        <input name="menu_type" type="hidden" value="classic"></input>_
                        <input name="browser" type="hidden" value=""></input>_
                        <input name="platform" type="hidden" value=""></input>_
                    </div>_
                </form>_
            </div>_
        </div>_
        <div id="pageContentContainer" class="pageElementTop">_
            <!--"kZd6CmmgS-q3ZJsbi_QXJmy4uIhbl0Pt05ddWHx3vcs"-->_
            <div id="pageLeft" class="pageElementLeft">_
                <!--"bhHbWVACRyHBE-MoOAfeLy6SUZbsJmGyCbT94cYBHHI"-->_
                <div id="pageLeftTop">
                </div>_
            </div>_
            <div id="pageContent" class="pageElementLeft">_
                <div id="featureBanner">
                </div>_
                <a name="mainContent" class="hidden">_
                </a>_
                <!--"up1YWWVw7bFlV69jn_wheiJ5MLDQ9_KdGWCUZ5gGeuw"-->_
                <div id="pageContentTop" class="pageElementTop">_
                </div>_
                <div id="contentSpacer_IE" class="pageElementTop">
                    <!--"WVhEeLYGpyH0bXmFoofJIUMWxdfkLBe5aUmIdmUfqiM"-->_
                    <!--"CKcFISCJjRLw3ii080mSqvobpMA3Z3OFHiqwurhqzcI"-->_
    }
    html_handler
}

#[must_use]
pub fn footer<'a>(html_handler: InElement<'a, InElement<'a, InElement<'a, InRoot<'a, Root<'a>>>>>, id: u64, subid: u64) -> InRoot<'a, Root<'a>> {
    html_extractor::html! {
                    <div id="pageFoot" class="pageElementTop">_
                        <div id="pageFootControls" class="pageElementTop">_
                            <div id="pageFootControlsLeft">_
                                <a href={&format!("?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N{subid:06},-Aimprint")} class="img img_arrowImprint pageElementLeft" id="pageFootControl_imp">
                                    "Impressum"
                                </a>_
                                <a href={&format!("?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N{subid:06},-Acontact")} class="img img_arrowContact pageElementLeft" id="pageFootControl_con">
                                    "Kontakt"
                                </a>_
                                <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft" id="pageFootControl_pri">
                                    "Drucken"
                                </a>_
                            </div>_
                            <div id="pageFootControlsRight">_
                                <a href="#top" class="img img_arrowUp pageElementRight" id="pageFootControl_up">_
                                </a>_
                            </div>_
                        </div>_
                    </div>_
                </div>_
                <div id="IEdiv">_
                </div>
                <!--"sA0YIGyByIKeA31YLo4xBo8n4XODq22IfHyrzzrnD-w"-->_
                <!--"em2y7JxbjqWZd3r7SQA-YKIJZsneemykpZ46ZXTq7Tw"-->_
                <!--"VwiU8OlvNnMu2C0d8thjT7A2X3pYuFyyhLNGOJ87AXc"-->_
                <div class="invAnchor">_
                    <a name="bottom" class="invAnchor">_
                    </a>_
                </div>_
            </body>
        </html>
    };
    html_handler
}
