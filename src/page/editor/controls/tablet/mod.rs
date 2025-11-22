use leptos::prelude::*;

use crate::{components::{button::format_tables::FormatTablesButton, tooltip::TooltipDirection}, page::editor::controls::tablet::format_buttons::EditTextButtons};
use crate::api::parser::Dialect;

mod format_buttons;

#[component]
pub fn TabletControls(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
) -> impl IntoView {
    
    view! {
        <div class="controls-container is-hidden-desktop is-hidden-mobile">
            <div class="is-flex is-justify-content-end py-3">
                <FormatTablesButton markdown tooltip_direction=TooltipDirection::Right />
            </div>
            <EditTextButtons markdown parser />
        </div>
    }
}