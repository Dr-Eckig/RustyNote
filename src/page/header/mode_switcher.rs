use leptos::prelude::*;
use leptos_use::use_media_query;

use crate::{Mode, components::{Color, Size, button::Button, icons::Icon}};

#[component]
pub fn ModeSwitcher(mode: RwSignal<Mode>) -> impl IntoView {
    
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
        <DesktopModeSwitcher mode />
        <TouchDeviceModeSwitcher mode />
	}
}

#[component]
fn DesktopModeSwitcher(mode: RwSignal<Mode>) -> impl IntoView {
    
    view! {
        <div class="buttons has-addons is-hidden-desktop my-auto">
			<Button 
				aria_label=String::from("Write Mode")
				icon=Icon::Write 
				color=Signal::derive(move || { if mode.get() == Mode::Write { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Write)
			/>
			<Button 
				aria_label=String::from("Read Mode")
				icon=Icon::Read 
				color=Signal::derive(move || { if mode.get() == Mode::Read { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Read) 
			/>
		</div>
    }
}

#[component]
fn TouchDeviceModeSwitcher(mode: RwSignal<Mode>) -> impl IntoView {
    
    view! {
        <div class="buttons has-addons is-hidden-touch my-auto">
			<Button 
				aria_label=String::from("Write Mode")
				icon=Icon::Write 
				color=Signal::derive(move || { if mode.get() == Mode::Write { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Write)
			/>
			<Button 
				aria_label=String::from("Split Mode")
				icon=Icon::Split 
				color=Signal::derive(move || { if mode.get() == Mode::Split { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Split) 
			/>
			<Button 
				aria_label=String::from("Read Mode")
				icon=Icon::Read 
				color=Signal::derive(move || { if mode.get() == Mode::Read { Color::Primary } else { Color::Light } })
				size=Size::Small
				on_click=move || mode.set(Mode::Read) 
			/>
		</div>
    }
}
