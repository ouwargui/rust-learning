use crate::{
    async_calls::AsyncCalls, async_todos::AsyncTodos, counter::Counter, home::Home,
    suspense::Suspense,
};

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
            <main class="bg-zinc-800 w-full min-h-screen flex flex-col items-center justify-center" >
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/counter" view=Counter />
                    <Route path="/async" view=AsyncCalls />
                    <Route path="/suspense" view=Suspense />
                    <Route path="/async_todos" view=AsyncTodos />
                </Routes>
            </main>
        </Router>
    }
}
