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
        let markdown = markdown.read();
        parser.with(|parser| parser.parse_markdown_to_html(&markdown))
    });

    view! {
        <div 
            class="column m-0 pr-0" 
            class:pl-0=mode.get() != Mode::Split 
            style="overflow-x: auto; max-width: 100%; text-overflow: break-word;"
        >
            <div 
                class="card content full-height p-5" 
                inner_html=move || parsed_markdown.get()
            /> 
        </div>
    }
}