use dioxus::prelude::*;
use tucant_types::vv::ActionRequest;

use crate::Route;

#[component]
pub fn NavbarLoggedOut() -> Element {
    rsx! {
            li { class: "nav-item",
                a {
                    class: "nav-link",
                    href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome",
                    "Startseite",
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href:"#", role:"button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    "VV"
                }
                ul { class:"dropdown-menu",
                    li {
                        Link {
                            to: Route::Vorlesungsverzeichnis {
                                vv: ActionRequest::parse("-AN07PBvMn59bWIkwI5kPIrV6ttS-nQO52gY48WnmIDWTv9PQsRceJIEekBMsiG7XrGxJxL6WmWMRCgv6ZdqcqJvgDTJ41d1yHBN12FkxT2-2R1XLasNa7As0AF4mdh2AohuT~wrzHUbQsFAkJJF23tlDnGaVBwg3B7S2UW-GrR0DSb24IOCR8EhR1~A__")
                            },
                            class: "dropdown-item bg-success",
                            "Vorlesungsverzeichnis"
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item", href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-ApmupJef8fipOyK7bl09ygBDteYMod2NhGXau-cCDtwF~ggV1owM8UGwi5QASFA9535nG6r-P3aBHfQ37AdKwI9XU3o5lAqbiph8OKOZdmIc2aQDE2rraXrtfetu-R36DT08Ilu6r2Xk5AWdXIpsk2mXXrhm2",
                            "Lehrveranstaltungssuche"
                        }
                    }
                    li {
                        a { class:"dropdown-item", href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N000000000000001,-N000385,",
                            "Raumsuche"
                        }
                    }
                    li {
                        Link {
                            to: Route::Vorlesungsverzeichnis {
                                vv: ActionRequest::parse("-AEv3p-ixRiFiH8xo86RrKK9uf72StPaSd2hoHwg~OCNTRSPUhnJzdfm2ekgyicwVEYv0sluY-Xc02k-Ok9YzmHJMN~8ZC5XqBZRJWAgaUOXj0O99z7w1fzIT-9l2n08Ivm2cZlRYRW-OLNzyzfekzeBlJcHKHIyvLHyZM1Gw7U0RN428Qtt79KAuiKQ__")
                            },
                            class: "dropdown-item bg-success",
                            "Aktuell - Sommersemester 2025"
                        }
                    }
                    li {
                        Link {
                            to: Route::Vorlesungsverzeichnis {
                                vv: ActionRequest::parse("-AV49BnEx08lkzBuUkpL1OiPOiy3h-IQL20ziRJFpOsEY2DC1~UykqG1eQ8bvz68jeZWo2btN4wItUnwQLaiKvRjVeU-HLKg-t83hCL4AmQ3H2Cn-as8NQ7ITOtoJXIQoEFvGuq6fgf7itKqWJGnHUuKsH5k3PgoK~dcpAqk7jG2vRTR7jTf6iDg8GoQ__")
                            }, class:"dropdown-item bg-success",
                            "Vorlesungsverzeichnis des WiSe 2024/25"
                        }
                    }
                    li {
                        Link {
                            to: Route::Vorlesungsverzeichnis {
                                vv: ActionRequest::parse("-Avn07o4KNB3FseLBaQAymKF-c26tMa37alo49WaWmPNAs6c~eB~9eUKwhY254n8VlPObojuspLhwsEVitzdGci5tybqHGcovSsTvIbYZMNs1-6N8WKN6ieWGsv4ScDLXw~hThHy7imvWnzEqAY-6kee5lmdm8GhDxStGxy5AwNpGJW73ftfmIvUZlgQ__")
                            }, class: "dropdown-item bg-success",
                            "Vorlesungsverzeichnis des SoSe2024"
                        }
                    }
                    li {
                        a { class:"dropdown-item", href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000463,-Avvarchivstart%2Ehtml",
                            "Archiv"
                        }
                    }
                }
            }
            li { class:"nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role:"button", "data-bs-toggle":"dropdown", "aria-expanded": "false",
                    "TUCaN-Account"
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml",
                            "TUCaN-Account"
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item", href: "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425,",
                            "Account anlegen"
                        }
                    }
                    li {
                        a { class:"dropdown-item", href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A",
                            "Passwort vergessen (nur für Bewerber/innen!)"
                        }
                    }
                }
            }
            li { class: "nav-item",
                a { class:"nav-link", href:"https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml",
                    "Hilfe"
                }
            }
    }
}
