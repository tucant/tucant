use tucant_types::LoginResponse;
use yew::{function_component, html, use_context, Html, UseStateHandle};

use crate::LoginComponent;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    html! {
        <nav class="navbar navbar-expand-xl bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">{ "TUCaN't" }</a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                    aria-controls="navbarSupportedContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon" />
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-xl-0">
                        if current_session.is_some() {
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Aktuelles" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Aktuelles" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Nachrichten" }
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
                                    { "VV" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
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
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Lehrveranstaltungssuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Raumsuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Aktuell - Wintersemester 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis Gasthörer_innen WiSe 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des SoSe 2024" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des WiSe 2023/24" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
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
                                    { "Stundenplan" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Stundenplan" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Tagesansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Wochenansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Monatsansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Export" }
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
                                    { "Veranstaltungen" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Veranstaltungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Module" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Veranstaltungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Wahlbereiche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#/registration/,-N000311,-A"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Anmeldung" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Mein aktueller Anmeldestatus" }
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
                                    { "Prüfungen" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Prüfungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Prüfungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Mein Prüfungsplan" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Semesterergebnisse" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Leistungsspiegel" }
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
                                    { "Service" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Service" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Persönliche Daten" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Dokumente" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Anträge" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Sperren" }
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
                                    { "Bewerbung" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Bewerbung" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Bewerbung" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Dokumente" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="#"
                                    data-bs-toggle="collapse"
                                    data-bs-target=".navbar-collapse.show"
                                >
                                    { "Hilfe" }
                                </a>
                            </li>
                        } else {
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="#"
                                    data-bs-toggle="collapse"
                                    data-bs-target=".navbar-collapse.show"
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
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
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
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Lehrveranstaltungssuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Raumsuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Aktuell - Wintersemester 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis Gasthörer_innen WiSe 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des SoSe 2024" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des WiSe 2023/24" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
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
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Account anlegen" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Passwort vergessen (nur für Bewerber/innen!)" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="#"
                                    data-bs-toggle="collapse"
                                    data-bs-target=".navbar-collapse.show"
                                >
                                    { "Hilfe" }
                                </a>
                            </li>
                        }
                    </ul>
                    if !current_session.is_some() {
                        <LoginComponent />
                    }
                </div>
            </div>
        </nav>
    }
}
