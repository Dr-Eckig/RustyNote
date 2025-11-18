use leptos::prelude::*;

use crate::{components::{Color, Size, button::Button, icons::Icon, tooltip::TooltipDirection}, page::controls::components::FormatTablesButton};

#[component]
pub fn MobileControls(markdown: RwSignal<String>, sidebar_open: RwSignal<bool>) -> impl IntoView {

    view! { 
        <div class="is-flex is-justify-content-space-between is-hidden-tablet px-5 pt-3">
            <Button 
                icon=Signal::derive(move || if sidebar_open.get() { Icon::ChevronLeft } else { Icon::ChevronRight })
                color=Color::Primary
                size=Size::Small
                on_click=move || sidebar_open.set(!sidebar_open.get())
            />
            <FormatTablesButton markdown tooltip_direction=TooltipDirection::Right />
        </div>
    }
}