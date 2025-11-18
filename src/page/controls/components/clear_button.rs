use leptos::prelude::*;

use crate::components::confirmation::Confirmation;

#[component]
pub fn ClearMarkdownButton(markdown: RwSignal<String>) -> impl IntoView {

    view! {
        <Confirmation 
            confirmation_text="Are you sure? This will reset the whole content." 
            on_confirmation=move || {
                markdown.set(String::new());
            }
        />
    }
}