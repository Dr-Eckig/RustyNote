use leptos::prelude::*;
use markdown::App;

pub fn main() {
    mount_to_body(move || view! { 
        <App />
    });
}
