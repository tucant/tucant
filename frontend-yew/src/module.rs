use gloo_net::http::Request;
use tucant_core::models::ModuleResponse;
use yew::prelude::*;
use yew::suspense::use_future;

#[function_component(ModuleComponent)]
pub fn module() -> HtmlResult {
    let module = use_future(|| async move {
        let module: ModuleResponse = Request::post("http://localhost:8080/module")
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

    Ok(html! {<div>{"Hello, "}{module.module.title.clone()}</div>})
}
