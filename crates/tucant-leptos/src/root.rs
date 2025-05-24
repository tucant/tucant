use leptos::{ev::Targeted, html::Input, prelude::*};

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <div class="container">
            <h1>{"Willkommen bei TUCaN't!"}</h1>
            <p>
                {"Du kannst gerne die "} <a href="https://tucant.github.io/tucant/" target="_blank">
                    {"Browsererweiterung herunterladen"}
                </a> {", falls Du diese noch nicht verwendest."}
            </p>
            <p>
                {"Der Quellcode dieses Projekts ist unter der AGPL-3.0 Lizenz auf "}
                <a href="https://github.com/tucant/tucant/" target="_blank">
                    {"GitHub"}
                </a> {" verfügbar."}
            </p>
            <p>
                {"Du kannst Dir deine "}
                <a href="#/registration/">{"anmeldbaren Module ansehen"}</a> {"."}
            </p>
        </div>
    }
}
