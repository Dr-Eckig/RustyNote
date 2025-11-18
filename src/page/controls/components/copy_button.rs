use leptoaster::expect_toaster;
use leptos::prelude::*;
use leptos_use::{use_clipboard, UseClipboardReturn};

use crate::components::{Color, Size, State, button::Button, icons::Icon, tooltip::{Tooltip, TooltipDirection}};

#[component]
pub fn CopyMarkdownButton(markdown: RwSignal<String>) -> impl IntoView {

    let UseClipboardReturn { is_supported, copy, .. } = use_clipboard();
    
    let toaster = expect_toaster();

    let copy_to_clipboard = move || {
        let content = markdown.get();
        copy(&content);
        toaster.success("Markdown copied to clipboard!");
    };

    view! {
        <Tooltip text=Signal::derive(move || if is_supported.get() { "Copy to Clipboard" } else { "Your Browser does not support copying" }) direction=TooltipDirection::Right>
            <Button 
                icon=Icon::Copy
                color=Color::White
                size=Size::Small 
                state=Signal::derive(move || if is_supported.get() { State::Normal } else { State::Disabled })
                on_click=move || copy_to_clipboard()
            />
        </Tooltip>
    }
}