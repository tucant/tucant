use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    html! {
        <>
        <nav class="navbar navbar-expand-lg navbar-light">
        <div class="container-fluid">
            <a class="navbar-brand" href="#">
                <img src="/TUCaNt.svg" alt="Logo" width="30" height="24" class="d-inline-block align-text-top" />
            {"TUCaN't"}
            </a>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                <a class={classes!("nav-link", matches!(routes.clone(), Route::Home).then_some("active"))} aria-current="page" href="#">{"Home"}</a>
                </li>
                <li class="nav-item">
                <a class="nav-link" href="#">{"Link"}</a>
                </li>
                <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    {"Dropdown"}
                </a>
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                    <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                    <li><hr class="dropdown-divider" /></li>
                    <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                </ul>
                </li>
                <li class="nav-item">
                <a class="nav-link disabled">{"Disabled"}</a>
                </li>
            </ul>
            <form class="d-flex" role="search">
                <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search" />
                <button class="btn btn-outline-success" type="submit">{"Search"}</button>
            </form>
            </div>
        </div>
        </nav>
        <main class="container">{
                match routes {
                    Route::Home => html! { <h1>{ "Home" }</h1> },
                    Route::Secure => html! {
                        <Secure />
                    },
                    Route::NotFound => html! { <h1>{ "404" }</h1> },
                }
            }</main>
    </>
    }
   
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
