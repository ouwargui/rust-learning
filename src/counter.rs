use leptos::{ev::MouseEvent, *};
use leptos_router::A;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let handleBlueClass = move || count.get() % 2 == 1;
    let handleRedClass = move || count.get() % 2 == 0;

    view! { cx,
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
        <A
            class="border border-zinc-700 rounded p-2 text-zinc-200 bg-indigo-500"
            href="/"
        >
            "Go back"
        </A>
    }
}
