use leptos::prelude::*;

use crate::Mode;
use crate::api::parser::Dialect;
use crate::page::editor::controls::{
    desktop::DesktopControls, mobile::MobileControls, tablet::TabletControls,
};

pub mod desktop;
pub mod mobile;
pub mod tablet;

#[component]
pub fn Controls(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
    sidebar_open: RwSignal<bool>,
    mode: RwSignal<Mode>,
) -> impl IntoView {
    view! {
        <div>
            <DesktopControls markdown parser mode />
            <TabletControls markdown parser mode />
        </div>
        <MobileControls markdown sidebar_open mode />
    }
}
