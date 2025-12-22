use leptos::prelude::*;
use markdown::App;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! {
            <App />
        }
    });
}
