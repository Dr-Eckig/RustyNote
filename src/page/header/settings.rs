use leptos::prelude::*;

use crate::components::{HelpModal, button::{copy::CopyButton, delete::DeleteButton, download::DownloadButton}, dialect::DialectSelect, dropdown::{DropdownButton, DropdownDirection}, icons::Icon, theme::ThemeSelect};
use crate::api::parser::Dialect;

#[component]
pub fn HeaderSettings(markdown: RwSignal<String>, parser: RwSignal<Dialect>) -> impl IntoView {
    
    view! {
        <DesktopSettings parser />
        <TouchDeviceSettings markdown parser />
    }
}

#[component]
fn DesktopSettings(parser: RwSignal<Dialect>) -> impl IntoView {

    view! {
        <div class="is-flex is-align-items-center is-hidden-touch">
			<DialectSelect parser />
			<div class="px-1"/>
			<ThemeSelect />
			<div class="px-1"/>
			<HelpModal />
		</div>
    }
}

#[component]
fn TouchDeviceSettings(markdown: RwSignal<String>, parser: RwSignal<Dialect>) -> impl IntoView {

    view! {
        <div class="is-flex is-align-items-center is-hidden-desktop">
			<DropdownButton aria_label=String::from("Settings") icon=Icon::Bars direction=DropdownDirection::Right>
				<div class="dropdown-item"> 
					<DialectSelect parser />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<ThemeSelect />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<HelpModal is_dropdown_item=true fullsize_button=true />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<CopyButton markdown=markdown.read_only() />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<DownloadButton markdown=markdown.read_only() />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<DeleteButton markdown />
				</div>
			</DropdownButton>
		</div>
    }
}