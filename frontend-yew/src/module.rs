use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn load_user() -> Option<User> {
    todo!()  // implementation omitted.
}

fn on_load_user_complete<F: FnOnce()>(_fn: F) {
    todo!()  // implementation omitted.
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // If a user is loaded, then we return it as Ok(user).
        Some(m) => Ok(m),
        None => {
            // When user is still loading, then we create a `Suspension`
            // and call `SuspensionHandle::resume` when data loading
            // completes, the component will be re-rendered
            // automatically.
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
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