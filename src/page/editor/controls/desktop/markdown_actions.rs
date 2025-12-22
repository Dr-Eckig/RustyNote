use leptos::prelude::*;

use crate::components::button::{copy::CopyButton, delete::DeleteButton, download::DownloadButton};

#[component]
pub fn ActionButtons(markdown: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="buttons">
            <CopyButton markdown=markdown.read_only() />
            <DownloadButton markdown=markdown.read_only() />
            <DeleteButton markdown />
        </div>
    }
}
