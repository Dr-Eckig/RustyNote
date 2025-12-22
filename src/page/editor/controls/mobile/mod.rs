use leptos::prelude::*;

use crate::Mode;
use crate::components::State;
use crate::components::{
    Color, Size,
    button::{Button, format_tables::FormatTablesButton},
    icons::Icon,
    tooltip::{Tooltip, TooltipDirection},
};
pub mod format_buttons;

#[component]
pub fn MobileControls(
    markdown: RwSignal<String>,
    sidebar_open: RwSignal<bool>,
    mode: RwSignal<Mode>,
) -> impl IntoView {
    let button_state = Signal::derive(move || {
        if mode.get() == Mode::Read {
            State::Disabled
        } else {
            State::Normal
        }
    });

    view! {
        <div class="is-flex is-justify-content-space-between is-hidden-tablet px-5 pt-3">
            <Tooltip text=String::from("❌ The Format Buttons are not available in Read Mode") is_hidden=Signal::derive(move || matches!(mode.get(), Mode::Write))>
                <Button
                    aria_label=Signal::derive(move || if sidebar_open.get() { "Close Sidebar" } else { "Open Sidebar" })
                    icon=Signal::derive(move || if sidebar_open.get() { Icon::ChevronLeft } else { Icon::ChevronRight })
                    color=Color::Primary
                    size=Size::Small
                    state=button_state
                    on_click=move || sidebar_open.set(!sidebar_open.get())
                />
            </Tooltip>
            <FormatTablesButton markdown tooltip_direction=TooltipDirection::Right />
        </div>
    }
}
