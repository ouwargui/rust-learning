mod app;
mod async_calls;
mod async_todos;
mod counter;
mod home;
mod suspense;

extern crate console_error_panic_hook;
use std::panic;

use app::*;
use leptos::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook)); // catch panics in `wasm-bindgen` callbacks
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
