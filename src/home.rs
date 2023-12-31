use leptos::*;
use leptos_router::A;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex gap-2">
            <A
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-cyan-500"
                href="/counter"
            >
                "Go to counter"
            </A>
            <A
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
                href="/async"
            >
                "Go to async calls"
            </A>
            <A
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-pink-500"
                href="/suspense"
            >
                "Go to suspense components"
            </A>
        </div>
        <div class="flex gap-2">
            <A
                class="border border-zinc-700 rounded p-2 text-zinc-200 bg-emerald-500"
                href="/async_todos"
            >
                "Go to async todo list"
            </A>
        </div>
    }
}
