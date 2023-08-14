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

    let handleBlueClass = move || count.get() % 2 == 1;
    let handleRedClass = move || count.get() % 2 == 0;

    view! { cx,
        <main class="bg-zinc-800 w-full min-h-screen flex flex-col items-center justify-center" >
            <span class="text-zinc-200">"Counter = " {count}</span>
            <button
                class="border border-zinc-700 p-2 rounded text-zinc-200"
                class=("bg-blue-600", handleBlueClass)
                class=("bg-red-600", handleRedClass)
                on:click=move |_: MouseEvent| {
                    set_count.update(|v| *v += 1);
                }
            >
                Increment
            </button>
        </main>
    }
}
