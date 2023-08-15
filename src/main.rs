mod app;
mod async_calls;
mod async_todos;
mod counter;
mod home;
mod suspense;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
