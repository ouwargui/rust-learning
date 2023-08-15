use crate::async_calls::AsyncCalls;
use crate::home::Home;
use crate::suspense::Suspense;

use leptos::*;
use leptos_router::{Route, Router, Routes};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/async" view=AsyncCalls />
                    <Route path="/suspense" view=Suspense />
                </Routes>
            </main>
        </Router>
    }
}
