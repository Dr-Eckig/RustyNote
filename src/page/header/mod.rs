mod logo;
mod mode_switcher;
mod settings;

use crate::page::header::{
    logo::LogoWithTitle, mode_switcher::ModeSwitcher, settings::HeaderSettings,
};
use leptos::prelude::*;

#[component]
pub fn Header(
    markdown: RwSignal<String>,
    mode: RwSignal<crate::Mode>,
    parser: RwSignal<crate::Dialect>,
) -> impl IntoView {
    view! {
        <header class="columns is-mobile header has-shadow-bottom m-0">
            <div class="column">
                <LogoWithTitle />
            </div>

            <div class="column is-flex is-justify-content-center is-align-items-center">
                <ModeSwitcher mode />
            </div>

            <div class="column is-flex is-justify-content-end is-align-items-center">
                <HeaderSettings markdown parser />
            </div>
        </header>
    }
}
