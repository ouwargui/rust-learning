use leptos::*;
use leptos_router::A;

#[component]
pub fn AnywhereElse(cx: Scope) -> impl IntoView {
    view! { cx,
      <main class="bg-zinc-800 w-full min-h-screen flex flex-col items-center justify-center" >
        <A
            class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
            href="/"
        >
            "Go back"
        </A>
      </main>
    }
}
