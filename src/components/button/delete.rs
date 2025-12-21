use leptos::prelude::*;

use crate::components::{Color, button::Button, confirmation::Confirmation, icons::Icon};

#[component]
pub fn DeleteButton(
    markdown: RwSignal<String>,
) -> impl IntoView {
    
    view! {
        <DesktopDeleteButton markdown />
        <TouchDeviceDeleteButton markdown />
    }
}

#[component]
fn DesktopDeleteButton(
    markdown: RwSignal<String>,
) -> impl IntoView {
    
    view! {
        <div class="is-hidden-touch">
            <Confirmation 
                confirmation_text="Are you sure? This will reset the whole content." 
                on_confirmation=move || {
                    markdown.set(String::new());
                }
            />
        </div>
    }
}

#[component]
fn TouchDeviceDeleteButton(
    markdown: RwSignal<String>,
) -> impl IntoView {
    
    view! {
        <div class="is-hidden-desktop">
            <Button
                aria_label=String::from("Clear Markdown")
                text="Clear Markdown"
                icon=Icon::Delete 
                color=Color::Transparent
                has_smaller_padding=true
                is_full_size=true
                on_click=move || markdown.set(String::new())
            />
        </div>
    }
}

