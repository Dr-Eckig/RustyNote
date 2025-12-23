use leptos::prelude::*;

use crate::Mode;
use crate::api::parser::Dialect;
use crate::{
    components::{button::format_tables::FormatTablesButton, tooltip::TooltipDirection},
    page::editor::controls::tablet::format_buttons::EditTextButtons,
};

mod format_buttons;

#[component]
pub fn TabletControls(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
    mode: RwSignal<Mode>,
) -> impl IntoView {
    view! {
        <div class="is-hidden-desktop is-hidden-mobile px-5">
            <div class="is-flex is-justify-content-end py-3">
                <FormatTablesButton markdown tooltip_direction=TooltipDirection::Right />
            </div>
            <EditTextButtons markdown parser mode />
        </div>
    }
}
