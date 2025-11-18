use leptos::prelude::*;

use crate::{Mode, api::{markdown_formatter::{handle_enter_for_lists, setup_shortcuts}, parser::Dialect}};
use crate::components::mobile::MobileSidebar;

#[component]
pub fn MarkdownEditor(
    markdown: RwSignal<String>,
    mode: RwSignal<Mode>,
    parser: RwSignal<Dialect>,
    mobile_sidebar_open: RwSignal<bool>,
) -> impl IntoView {

    setup_shortcuts(markdown);

    let parsed_markdown = Signal::derive(move || {
        parser.read().parse_markdown_to_html(&markdown.read())
    });
        
    view! {
        <main class="columns is-mobile is-flex-grow-1 px-5 m-0" style="overflow: hidden;">
            { move || (mode.get() == Mode::Write || mode.get() == Mode::Split).then(|| 
                view! {
                    <div class="column is-narrow is-hidden-tablet px-0">
                        <MobileSidebar markdown parser sidebar_open=mobile_sidebar_open />
                    </div>
                    <div 
                        class="column pl-0 editor" 
                        class:has-sidebar=move || mobile_sidebar_open.get()
                        class:pr-0=mode.get() != Mode::Split
                    >
                        <textarea 
                            id="markdown-textarea"
                            class="textarea has-fixed-size card full-height is-family-monospace p-5"
                            placeholder="Write your Markdown here..."
                            prop:value=markdown
                            on:input=move |ev| markdown.set(event_target_value(&ev))
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    ev.prevent_default();
                                    handle_enter_for_lists();
                                }
                                // if ev.key() == "Tab" {
                                //     ev.prevent_default();
                                //     handle_tab();
                                // }
                            }
                        />
                    </div>
                }
            )}

            { move || (mode.get() == Mode::Read || mode.get() == Mode::Split).then(|| 
                view! {
                    <div class="column content pr-0" class:pl-0=mode.get() != Mode::Split style="overflow-x: auto; max-width: 100%; text-overflow: break-word;">
                        <div 
                            class="card full-height p-5" 
                            inner_html=move || parsed_markdown.get()
                        /> 
                    </div>
                }
            )}
        </main>
    }
}
