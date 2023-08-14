use leptos::{*, ev::MouseEvent};
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
    let (count, set_count) = create_signal(cx, 0);

    let increment = move |_: MouseEvent| set_count.update(|v| *v += 1);

    view! { cx,
        <main class="bg-red-200 w-full min-h-screen flex flex-col items-center justify-center" >
            <span>Counter = {count}</span>
            <button on:click=increment class="border border-zinc-700 p-2 rounded bg-blue-300">Increment</button>
        </main>
    }
}
