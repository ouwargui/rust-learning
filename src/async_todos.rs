use leptos::{
    ev::{MouseEvent, SubmitEvent},
    html::Input,
    *,
};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

type TodoList = Vec<Todo>;

impl Todo {
    fn new(id: i32, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }
}

async fn fetch_todos() -> TodoList {
    let todos = tauri::invoke::<_, TodoList>("fetch_todos", &())
        .await
        .unwrap();
    todos
}

#[derive(Serialize, Deserialize)]
struct AddTodo {
    todo: Todo,
}

async fn add_todo(todo: Todo) -> TodoList {
    let res = tauri::invoke::<AddTodo, TodoList>("add_todo", &AddTodo { todo })
        .await
        .unwrap();
    res
}

#[derive(Serialize, Deserialize)]
struct RemoveTodo {
    id: i32,
}

async fn remove_todo(id: i32) -> TodoList {
    let res = tauri::invoke::<RemoveTodo, TodoList>("remove_todo", &RemoveTodo { id })
        .await
        .unwrap();
    res
}

#[component]
pub fn AsyncTodos(cx: Scope) -> impl IntoView {
    let (_counter, _set_counter) = create_signal(cx, 0);
    let todo_title_input_ref = create_node_ref::<Input>(cx);
    let async_todos = create_resource(cx, || (), |_| async move { fetch_todos().await });

    let todos_result = move || {
        async_todos
            .read(cx)
            .map(|todos| todos)
            // This loading state will only show before the first load
            .unwrap_or_else(|| vec![].into())
    };

    let add_todo_action = create_action(cx, move |input: &String| {
        let input = input.to_owned();
        let todos_len = todos_result().len();
        let todo = Todo::new(todos_len.try_into().unwrap(), input, false);
        async move {
            add_todo(todo).await;
            async_todos.refetch();
        }
    });

    let add_todo = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = todo_title_input_ref.get().expect("input to exist");
        add_todo_action.dispatch(input.value());
        input.set_value("");
    };

    let remove_todo_action = create_action(cx, move |todo_id: &i32| {
        let todo_id = todo_id.to_owned();
        async move {
            remove_todo(todo_id).await;
            async_todos.refetch();
        }
    });

    let remove_todo = move |ev: MouseEvent, todo_id: i32| {
        ev.prevent_default();
        remove_todo_action.dispatch(todo_id);
    };

    view! { cx,
        <form
            class="flex flex-col gap-4 min-h-screen py-4"
            on:submit=add_todo
        >
            <input
                class="border border-zinc-200 rounded-md p-2 bg-inherit outline-none text-zinc-200"
                type="text"
                placeholder="What needs to be done?"
                node_ref=todo_title_input_ref
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
                                    <button
                                        class="ml-2 text-zinc-200"
                                        type="button"
                                        on:click=move |ev| remove_todo(ev, todo.id)
                                    >
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
