use leptoaster::expect_toaster;
use leptos::prelude::*;
use leptos_use::{UseClipboardReturn, use_clipboard};

use crate::components::{
    Color, Size, State,
    button::Button,
    icons::Icon,
    tooltip::{Tooltip, TooltipDirection},
};

#[component]
pub fn CopyButton(markdown: ReadSignal<String>) -> impl IntoView {
    let UseClipboardReturn {
        is_supported, copy, ..
    } = use_clipboard();

    let toaster = expect_toaster();

    let copy_to_clipboard = move || {
        let content = markdown.get();
        copy(&content);
        toaster.success("Markdown copied to clipboard!");
    };

    view! {
        <DesktopCopyButton
            is_supported=is_supported
            copy_to_clipboard=copy_to_clipboard.clone()
        />
        <TouchDeviceCopyButton
            is_supported=is_supported
            copy_to_clipboard
        />
    }
}

#[component]
fn DesktopCopyButton(
    is_supported: Signal<bool>,
    copy_to_clipboard: impl Fn() + Send + 'static,
) -> impl IntoView {
    view! {
        <div class="is-hidden-touch">
            <Tooltip text=Signal::derive(move || if is_supported.get() { "Copy to Clipboard" } else { "Your Browser does not support copying" }) direction=TooltipDirection::Right>
                <Button
                    aria_label=String::from("Copy to Clipboard")
                    icon=Icon::Copy
                    color=Color::White
                    size=Size::Small
                    state=Signal::derive(move || if is_supported.get() { State::Normal } else { State::Disabled })
                    on_click=move || copy_to_clipboard()
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn TouchDeviceCopyButton(
    is_supported: Signal<bool>,
    copy_to_clipboard: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="is-hidden-desktop">
            <Button
                aria_label=String::from("Copy to Clipboard")
                text="Copy Markdown"
                icon=Icon::Copy
                color=Color::Transparent
                state=Signal::derive(move || if is_supported.get() { State::Normal } else { State::Disabled })
                has_smaller_padding=true
                is_full_size=true
                on_click=move || copy_to_clipboard()
            />
        </div>
    }
}
