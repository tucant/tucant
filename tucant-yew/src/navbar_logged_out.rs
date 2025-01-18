use yew::{function_component, html, Html};

#[function_component(NavbarLoggedOut)]
pub fn navbar_logged_out() -> Html {
    html! {
        <>
            <li class="nav-item">
                <a
                    class="nav-link"
                    href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome"
                >
                    { "Startseite" }
                </a>
            </li>
            <li class="nav-item dropdown">
                <a
                    class="nav-link dropdown-toggle"
                    href="#"
                    role="button"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                >
                    { "VV" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AN07PBvMn59bWIkwI5kPIrV6ttS-nQO52gY48WnmIDWTv9PQsRceJIEekBMsiG7XrGxJxL6WmWMRCgv6ZdqcqJvgDTJ41d1yHBN12FkxT2-2R1XLasNa7As0AF4mdh2AohuT~wrzHUbQsFAkJJF23tlDnGaVBwg3B7S2UW-GrR0DSb24IOCR8EhR1~A__"
                        >
                            { "Vorlesungsverzeichnis" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-ApmupJef8fipOyK7bl09ygBDteYMod2NhGXau-cCDtwF~ggV1owM8UGwi5QASFA9535nG6r-P3aBHfQ37AdKwI9XU3o5lAqbiph8OKOZdmIc2aQDE2rraXrtfetu-R36DT08Ilu6r2Xk5AWdXIpsk2mXXrhm2"
                        >
                            { "Lehrveranstaltungssuche" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N000000000000001,-N000385,"
                        >
                            { "Raumsuche" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A1UOJsw28R2k1y54teJLFI0AmVf~SynJG0lPJT4hWx3Pd2pgi2uLxxMBsS2cjb~FIP7H1vkneIgJXzeLZHmQnsV2wfuTkuB4oW5p-MMXJPojkAa33-gGvV1gbDS6A1OPsznmo~6xd~GSRxk3YFVK7crzHm1Yf8HKyXwC78ZIxnS-9~tB4L3Ul6uEkJQ__"
                        >
                            { "Aktuell - Wintersemester 2024/25" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Avg592S~LGYOfjtSndrtNxlVR2J9kIS7xmljoREqXTgcF6XcTMM0rTR5IBrCNWEp5sJpEsOljcv8c5N2aCILh9N6kOUfFFfGbQ~qQEtepIFPZO98~n7G8X0qpH2kWNuRmVW~qMPTY-HcdOhKUVDhM1X4owra8caR3S7MnROHEOzLqdWa5Zm2awq3Qag__"
                        >
                            { "Vorlesungsverzeichnis des SoSe2024" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AzMwKIbEl8fKxOyceOeUbGpsq~PU6GV3dKdLTh7n2lINJBcr7xxS4gwjhqFXEyi0GARw-A1oAVkgvWBa7dbFXV18fXBZ9oj3cco18MY5ZKrU7wq7~6IDot4aIipfyrvFSvGOTXmx3Me~ft-AjiQIqbQBGUtsEdzIeC64v3j3UqYqCSV2wj1JtwsguBw__"
                        >
                            { "Vorlesungsverzeichnis des WiSe 2023/24" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000463,-Avvarchivstart%2Ehtml"
                        >
                            { "Archiv" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a
                    class="nav-link dropdown-toggle"
                    href="#"
                    role="button"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                >
                    { "TUCaN-Account" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml"
                        >
                            { "TUCaN-Account" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425,"
                        >
                            { "Account anlegen" }
                        </a>
                    </li>
                    <li>
                        <a
                            class="dropdown-item"
                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A"
                        >
                            { "Passwort vergessen (nur für Bewerber/innen!)" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item">
                <a
                    class="nav-link"
                    href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml"
                >
                    { "Hilfe" }
                </a>
            </li>
        </>
    }
}
