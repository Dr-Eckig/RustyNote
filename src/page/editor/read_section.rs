use leptos::prelude::*;
use crate::api::parser::Dialect;
use crate::Mode;

#[component]
pub fn ReadSection(
    markdown: ReadSignal<String>,
    parser: ReadSignal<Dialect>,
    mode: ReadSignal<Mode>,
) -> impl IntoView {

    let parsed_markdown = Signal::derive(move || {
        parser.read().parse_markdown_to_html(&markdown.read())
    });

    view! {
        <div 
            class="column content m-0 pr-0" 
            class:pl-0=mode.get() != Mode::Split 
            style="overflow-x: auto; max-width: 100%; text-overflow: break-word;"
        >
            <div 
                class="card full-height p-5" 
                inner_html=move || parsed_markdown.get()
            /> 
        </div>
    }
}