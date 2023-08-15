use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

async fn fetch_todos() -> Vec<Todo> {
    let todos_from_js = invoke("fetch_todos", JsValue::UNDEFINED).await;
    let todos: Vec<Todo> = serde_wasm_bindgen::from_value(todos_from_js).unwrap();
    log!("todos: {:?}", todos);
    todos
}

#[component]
pub fn AsyncTodos(cx: Scope) -> impl IntoView {
    let async_todos = create_resource(cx, || (), |_| async move { fetch_todos().await });

    let todos_result = move || {
        async_todos
            .read(cx)
            .map(|todos| todos)
            // This loading state will only show before the first load
            .unwrap_or_else(|| vec![].into())
    };

    view! { cx,
        <form class="flex flex-col gap-4 min-h-screen py-4">
            <input
                class="border border-zinc-200 rounded-md p-2 bg-inherit outline-none text-zinc-200"
                type="text"
                placeholder="What needs to be done?"
            />
            <div class="text-zinc-200 ">
                <h1 class="text-lg">"Your tasks:"</h1>
                <ul class="list-disc">
                    {move || todos_result().into_iter()
                        .map(|todo| view! { cx,
                            <li>
                                <div class="flex flex-row justify-between">
                                    <span>
                                        {todo.title}
                                    </span>
                                    <button class="ml-2 text-zinc-200" type="button">
                                        "X"
                                    </button>
                                </div>
                            </li>
                        })
                        .collect_view(cx)}
                </ul>
            </div>
        </form>
    }
}
