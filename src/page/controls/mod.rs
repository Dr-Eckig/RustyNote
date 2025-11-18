mod components;
mod mobile_controls;

pub use mobile_controls::MobileControls;

use leptos::prelude::*;
use crate::api::parser::Dialect;
use crate::components::tooltip::TooltipDirection;
use crate::page::controls::components::{EditTextButtons, CopyMarkdownButton, DownloadButton, ClearMarkdownButton, FormatTablesButton};

#[component]
pub fn Controls(
    markdown: RwSignal<String>, 
    parser: RwSignal<Dialect>,
) -> impl IntoView {
    
    view! {
        <div class="is-flex-desktop is-justify-content-space-between is-hidden-mobile pt-4 px-5">
            <div class="is-flex-desktop is-align-items-center is-justify-content-space-between pb-0">
                <EditTextButtons markdown parser />
                <div class="is-hidden-touch">
                    <FormatTablesButton markdown tooltip_direction=TooltipDirection::Left />
                </div>
            </div>
            <div class="is-flex is-justify-content-end is-hidden-desktop pt-3">
                <FormatTablesButton markdown tooltip_direction=TooltipDirection::Right />
            </div>
            <div class="is-flex is-align-items-center is-justify-content-end is-hidden-touch">
                <div class="buttons">
                    <CopyMarkdownButton markdown />
                    <DownloadButton markdown />
                    <ClearMarkdownButton markdown />
                </div>
            </div>
        </div>
    }
}
