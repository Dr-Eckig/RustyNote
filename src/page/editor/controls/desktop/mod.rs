use crate::Mode;
use crate::{
    api::parser::Dialect,
    page::editor::controls::desktop::{
        format_buttons::EditTextButtons, markdown_actions::ActionButtons,
    },
};
use leptos::prelude::*;

mod format_buttons;
mod markdown_actions;

#[component]
pub fn DesktopControls(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
    mode: RwSignal<Mode>,
) -> impl IntoView {
    view! {
        <div class="controls-container is-flex is-justify-content-space-between is-hidden-touch pt-3 px-0">
            <EditTextButtons markdown parser mode />
            <ActionButtons markdown />
        </div>
    }
}
