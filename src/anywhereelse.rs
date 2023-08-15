use leptos::{ev::MouseEvent, *};
use leptos_router::A;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn load_asynchronous() -> String {
    invoke("load_asynchronous", JsValue::UNDEFINED)
        .await
        .as_string()
        .unwrap()
}

#[component]
pub fn AnywhereElse(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (apiMsg, set_api_msg) = create_signal(cx, String::new());

    let async_call = move |_: MouseEvent| {
        spawn_local(async move {
            let msg = invoke("load_asynchronous", JsValue::UNDEFINED)
                .await
                .as_string()
                .unwrap();

            set_api_msg.set(msg);
        })
    };

    let async_data = create_resource(
        cx,
        move || count.get(),
        // every time `count` changes, this will run
        |_| async move { load_asynchronous().await },
    );

    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let loading = async_data.loading();
    let is_loading = move || if loading.get() { "Loading..." } else { "Idle." };

    view! { cx,
      <main class="bg-zinc-800 w-full min-h-screen flex flex-col items-center justify-center gap-4">
        <div class="flex flex-col gap-2">
            <span class="text-zinc-200">"Message from API: " {apiMsg}</span>
            <button
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
                on:click=async_call
            >
                "Fetch something"
            </button>
        </div>
        <div class="flex flex-col gap-2">
            <span class="text-zinc-200">"Fetch status: " {async_result}</span>
            <span class="text-zinc-200">"Status: " {is_loading}</span>
            <button
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
                on:click=move |_| set_count.update(|prev| *prev += 1)
            >
                "Fetch again with status"
            </button>
        </div>
        <A
            class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
            href="/"
        >
            "Go back"
        </A>
      </main>
    }
}
