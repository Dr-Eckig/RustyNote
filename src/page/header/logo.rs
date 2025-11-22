use leptos::prelude::*;

use crate::components::logo::Logo;

#[component]
pub fn LogoWithTitle() -> impl IntoView {
    
    view! {
        <div class="is-flex is-justify-content-start is-align-items-center">
            <Logo />
            <p class="title is-4 is-hidden-mobile has-text-primary is-family-monospace is-flex is-align-items-center p-2">
                "RustyNote"
            </p>
        </div>
    }
}