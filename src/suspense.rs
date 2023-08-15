use leptos::{Suspense as LeptosSuspense, *};
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
pub fn Suspense(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (count2, set_count2) = create_signal(cx, 0);

    view! { cx,
        <main class="bg-zinc-800 w-full min-h-screen flex flex-col items-center justify-center gap-4">
            <div class="flex flex-row items-center gap-4">
                <Comp1 count=count />
                <button
                    class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
                    on:click=move |_| set_count.update(|prev| *prev += 1)
                >
                    "Fetch 1"
                </button>
            </div>
            <div class="flex flex-row items-center gap-4">
                <Comp2 count=count2 />
                <button
                    class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
                    on:click=move |_| set_count2.update(|prev| *prev += 1)
                >
                    "Fetch 2"
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

#[component]
pub fn Comp1(cx: Scope, count: ReadSignal<i32>) -> impl IntoView {
    let a = create_resource(
        cx,
        move || count.get(),
        |_| async move { load_asynchronous().await },
    );

    view! { cx,
        <LeptosSuspense
            fallback=move || view! { cx, <p class="text-zinc-200">"Loading 1..."</p> }
        >
            {move || {
                a.read(cx)
                    .map(|_| view! { cx, <p class="text-zinc-200">"Loaded 1!"</p> })
            }}
        </LeptosSuspense>
    }
}

#[component]
pub fn Comp2(cx: Scope, count: ReadSignal<i32>) -> impl IntoView {
    let b = create_resource(
        cx,
        move || count.get(),
        |_| async move { load_asynchronous().await },
    );

    view! { cx,
        <LeptosSuspense
            fallback=move || view! { cx, <p class="text-zinc-200">"Loading 2.."</p> }
        >
            {move || {
                b.read(cx)
                    .map(|_| view! { cx, <p class="text-zinc-200">"Loaded 2!"</p> })
            }}
        </LeptosSuspense>
    }
}
