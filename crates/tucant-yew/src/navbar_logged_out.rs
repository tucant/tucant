use tucant_types::vv::ActionRequest;
use yew::{Html, classes, function_component};
use yew_router::prelude::Link;

use crate::Route;

#[function_component(NavbarLoggedOut)]
pub fn navbar_logged_out() -> Html {
    ::yew::html! {
        <>
            <li class="nav-item">
                <a class="nav-link" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">
                    { "Startseite" }
                </a>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "VV" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <Link<Route> to={Route::Vorlesungsverzeichnis {
                            vv: ActionRequest::parse("-AN07PBvMn59bWIkwI5kPIrV6ttS-nQO52gY48WnmIDWTv9PQsRceJIEekBMsiG7XrGxJxL6WmWMRCgv6ZdqcqJvgDTJ41d1yHBN12FkxT2-2R1XLasNa7As0AF4mdh2AohuT~wrzHUbQsFAkJJF23tlDnGaVBwg3B7S2UW-GrR0DSb24IOCR8EhR1~A__")
                        }} classes={classes!("dropdown-item", "bg-success-subtle")}>
                            { "Vorlesungsverzeichnis" }
                        </Link<Route>>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-ApmupJef8fipOyK7bl09ygBDteYMod2NhGXau-cCDtwF~ggV1owM8UGwi5QASFA9535nG6r-P3aBHfQ37AdKwI9XU3o5lAqbiph8OKOZdmIc2aQDE2rraXrtfetu-R36DT08Ilu6r2Xk5AWdXIpsk2mXXrhm2">
                            { "Lehrveranstaltungssuche" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N000000000000001,-N000385,">
                            { "Raumsuche" }
                        </a>
                    </li>
                    <li>
                        <Link<Route> to={Route::Vorlesungsverzeichnis {
                            vv: ActionRequest::parse("-AEv3p-ixRiFiH8xo86RrKK9uf72StPaSd2hoHwg~OCNTRSPUhnJzdfm2ekgyicwVEYv0sluY-Xc02k-Ok9YzmHJMN~8ZC5XqBZRJWAgaUOXj0O99z7w1fzIT-9l2n08Ivm2cZlRYRW-OLNzyzfekzeBlJcHKHIyvLHyZM1Gw7U0RN428Qtt79KAuiKQ__")
                        }} classes={classes!("dropdown-item", "bg-success-subtle")}>
                            { "Aktuell - Sommersemester 2025" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Vorlesungsverzeichnis {
                            vv: ActionRequest::parse("-AV49BnEx08lkzBuUkpL1OiPOiy3h-IQL20ziRJFpOsEY2DC1~UykqG1eQ8bvz68jeZWo2btN4wItUnwQLaiKvRjVeU-HLKg-t83hCL4AmQ3H2Cn-as8NQ7ITOtoJXIQoEFvGuq6fgf7itKqWJGnHUuKsH5k3PgoK~dcpAqk7jG2vRTR7jTf6iDg8GoQ__")
                        }} classes={classes!("dropdown-item", "bg-success-subtle")}>
                            { "Vorlesungsverzeichnis des WiSe 2024/25" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Vorlesungsverzeichnis {
                            vv: ActionRequest::parse("-Avn07o4KNB3FseLBaQAymKF-c26tMa37alo49WaWmPNAs6c~eB~9eUKwhY254n8VlPObojuspLhwsEVitzdGci5tybqHGcovSsTvIbYZMNs1-6N8WKN6ieWGsv4ScDLXw~hThHy7imvWnzEqAY-6kee5lmdm8GhDxStGxy5AwNpGJW73ftfmIvUZlgQ__")
                        }} classes={classes!("dropdown-item", "bg-success-subtle")}>
                            { "Vorlesungsverzeichnis des SoSe2024" }
                        </Link<Route>>
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000463,-Avvarchivstart%2Ehtml">
                            { "Archiv" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "TUCaN-Account" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml">
                            { "TUCaN-Account" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425,">
                            { "Account anlegen" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A">
                            { "Passwort vergessen (nur für Bewerber/innen!)" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item">
                <a class="nav-link" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml">
                    { "Hilfe" }
                </a>
            </li></>
    }
}
