use leptoaster::expect_toaster;
use leptos::prelude::*;
use leptos_use::{UseClipboardReturn, use_clipboard, use_media_query};

use crate::api::download::download_file;
use crate::components::theme::ThemeSelect;
use crate::components::{Color, HelpModal, State};
use crate::components::logo::Logo;
use crate::components::select::Select;
use crate::components::dropdown::{DropdownButton, DropdownDirection};
use crate::components::{button::Button, icons::Icon, Size, };
use crate::api::parser::Dialect;
use crate::Mode;

#[component]
pub fn Header(
	markdown: RwSignal<String>,
	mode: RwSignal<Mode>,
	parser: RwSignal<Dialect>,
) -> impl IntoView {

    view! {
		<header class="columns is-mobile header has-shadow-bottom m-0"> 
			<div class="column">
				<div class="is-flex is-justify-content-start is-align-items-center">
					<Logo />
					<p class="title is-4 is-hidden-mobile has-text-primary is-family-monospace is-flex is-align-items-center p-2">
						"RustyNote"
					</p>
				</div>
			</div>
			
			<div class="column is-flex is-justify-content-center is-align-items-center">
				<ModeButtons mode />
			</div>

			<div class="column is-flex is-justify-content-end is-align-items-center">
				<SettingsButtons markdown parser />
			</div>
		</header>
	}
}

#[component]
fn ModeButtons(mode: RwSignal<Mode>) -> impl IntoView {

	let is_touch_device = use_media_query("(max-width: 1023px)");

	Effect::new(move || {
		let is_touch = is_touch_device.get();

		if is_touch {
			mode.set(Mode::Write);
		} else {
			mode.set(Mode::Split);
		}
	});
	
	view! {
		<div class="buttons has-addons is-hidden-touch my-auto">
			<Button 
				icon=Icon::Write 
				color=Signal::derive(move || { if mode.get() == Mode::Write { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Write)
			/>
			<Button 
				icon=Icon::Split 
				color=Signal::derive(move || { if mode.get() == Mode::Split { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Split) 
			/>
			<Button 
				icon=Icon::Read 
				color=Signal::derive(move || { if mode.get() == Mode::Read { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Read) 
			/>
		</div>

		<div class="buttons has-addons is-hidden-desktop">
			<Button 
				icon=Icon::Write 
				color=Signal::derive(move || { if mode.get() == Mode::Write { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Write)
			/>
			<Button 
				icon=Icon::Read 
				color=Signal::derive(move || { if mode.get() == Mode::Read { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Read) 
			/>
		</div>
	}
}

#[component]
fn SettingsButtons(markdown: RwSignal<String>, parser: RwSignal<Dialect>,) -> impl IntoView {

	let UseClipboardReturn { is_supported, copy, .. } = use_clipboard();
    
    let toaster = expect_toaster();

    let copy_to_clipboard = move || {
        let content = markdown.get();
        copy(&content);
        toaster.success("Markdown copied to clipboard!");
    };

	let download_markdown = move || {
        let content = markdown.get();
        download_file(content, "markdown.md");
    };
	
	view! {
		<div class="is-flex is-align-items-center is-hidden-touch">
			<Select 
				icon=Icon::Markdown 
				options=vec![String::from("Common"), String::from("GitHub")] 
				prop_value=Signal::derive(move || parser.get().to_string())
				on_change=move |value: String| {
					parser.set(value.parse().unwrap_or(Dialect::Common));
				}
			/>
			<div class="px-1"/>
			<ThemeSelect />
			<div class="px-1"/>
			<HelpModal />
		</div>

		<div class="is-flex is-align-items-center is-hidden-desktop">
			<DropdownButton icon=Icon::Bars direction=DropdownDirection::Right>
				<div class="dropdown-item"> 
					<Select 
						icon=Icon::Markdown 
						options=vec![String::from("Common"), String::from("GitHub")] 
						prop_value=Signal::derive(move || parser.get().to_string())
						on_change=move |value: String| {
							parser.set(value.parse().unwrap_or(Dialect::Common));
						}
					/>
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<ThemeSelect />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<HelpModal show_text=true fullsize_button=true />
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<Button 
						text="Copy Markdown"
						icon=Icon::Copy
						color=Color::Transparent
						state=Signal::derive(move || if is_supported.get() { State::Normal } else { State::Disabled })
						has_smaller_padding=true
						is_full_size=true
						on_click=move || copy_to_clipboard()
					/>
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<Button
						text="Download Markdown"
						icon=Icon::Download 
						color=Color::Transparent
						has_smaller_padding=true
						is_full_size=true
						on_click=move || download_markdown()
					/>
				</div>
				<hr class="dropdown-divider" />
				<div class="dropdown-item"> 
					<Button
						text="Clear Markdown"
						icon=Icon::Delete 
						color=Color::Transparent
						has_smaller_padding=true
						is_full_size=true
						on_click=move || markdown.set(String::new())
					/>
				</div>
			</DropdownButton>
		</div>
	}
}
