use crate::{
    api::download::download_file,
    components::{
        Color, Size,
        button::Button,
        icons::Icon,
        tooltip::{Tooltip, TooltipDirection},
    },
};
use leptos::prelude::*;

#[component]
pub fn DownloadButton(markdown: ReadSignal<String>) -> impl IntoView {
    let download_markdown = move || {
        let content = markdown.get();
        download_file(content, "markdown.md");
    };

    view! {
        <DesktopDownloadButton download_markdown=download_markdown />
        <TouchDeviceDownloadButton download_markdown />
    }
}

#[component]
fn DesktopDownloadButton(download_markdown: impl Fn() + Send + 'static) -> impl IntoView {
    view! {
        <div class="is-hidden-touch">
            <Tooltip text="Download Markdown" direction=TooltipDirection::Right>
                <Button
                    aria_label=String::from("Download Markdown")
                    icon=Icon::Download
                    color=Color::White
                    size=Size::Small
                    on_click=move || download_markdown()
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn TouchDeviceDownloadButton(download_markdown: impl Fn() + Send + 'static) -> impl IntoView {
    view! {
        <div class="is-hidden-desktop">
            <Button
                aria_label=String::from("Download Markdown")
                text="Download Markdown"
                icon=Icon::Download
                color=Color::Transparent
                has_smaller_padding=true
                is_full_size=true
                on_click=move || download_markdown()
            />
        </div>
    }
}
