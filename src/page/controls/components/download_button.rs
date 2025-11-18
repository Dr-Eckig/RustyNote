use leptos::prelude::*;

use crate::{api::download::download_file, components::{Color, Size, button::Button, icons::Icon, tooltip::{Tooltip, TooltipDirection}}};

#[component]
pub fn DownloadButton(markdown: RwSignal<String>) -> impl IntoView {

    let download_markdown = move || {
        let content = markdown.get();
        download_file(content, "markdown.md");
    };

    view! {
        <Tooltip text="Download Markdown" direction=TooltipDirection::Right>
            <Button
                icon=Icon::Download 
                color=Color::White
                size=Size::Small 
                on_click=move || download_markdown()
            />
        </Tooltip>
    }
}