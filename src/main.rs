mod async_calls;
mod app;
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
