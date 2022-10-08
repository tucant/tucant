use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult, use_future};
use gloo_net::http::Request;

#[derive(Debug)]
struct User {
    name: String,
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let user = use_future(|| async move {
        let module: () = Request::post("http://localhost:8080/module")
            .json("AAFZS0zPS1I=")
            .unwrap()
            .header("x-csrf-protection", "tucant")
            .credentials(gloo_net::http::RequestCredentials::Include)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
            module
    })?;

    Ok(html! {<div>{"Hello, "}</div>})
}

#[function_component(Module)]
pub fn module() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}