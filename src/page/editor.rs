use leptos::prelude::*;
use wasm_bindgen::JsCast;

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

    let textarea_ref = NodeRef::new();
    let scroll = RwSignal::new(0.0);
        
    view! {
        <main class="columns is-mobile is-flex-grow-1 px-5 m-0" style="overflow: hidden;">
            { move || (mode.get() == Mode::Write || mode.get() == Mode::Split).then(|| 
                view! {
                    <div class="column is-narrow is-hidden-tablet px-0">
                        <MobileSidebar markdown parser sidebar_open=mobile_sidebar_open />
                    </div>
                    <div class="column is-narrow is-hidden-mobile">
                        <LineColumn markdown scroll=scroll.read_only() />
                    </div>
                    <div 
                        class="column pl-0 editor" 
                        class:has-sidebar=move || mobile_sidebar_open.get()
                        class:pr-0=mode.get() != Mode::Split
                    >
                        <textarea 
                            id="markdown-textarea"
                            class="textarea has-fixed-size card full-height p-5"
                            placeholder="Write your Markdown here..."
                            node_ref=textarea_ref
                            prop:value=markdown
                            on:input=move |ev| markdown.set(event_target_value(&ev))
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    ev.prevent_default();                        
                                    markdown.set(handle_enter_for_lists());
                                    if let Some(textarea) = textarea_ref.get() {
                                        markdown.set(textarea.value());
                                        scroll.set(textarea.scroll_top().into());
                                    }
                                }
                                // if ev.key() == "Tab" {
                                //     ev.prevent_default();
                                //     handle_tab();
                                // }
                            }
                            on:scroll=move |ev| {
                                if let Some(target) = ev.target() {
                                    if let Ok(textarea) = target.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                        scroll.set(textarea.scroll_top().into());
                                    }
                                }
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

#[component]
fn LineColumn(markdown: RwSignal<String>, scroll: ReadSignal<f64>) -> impl IntoView {
    
    let line_count = Signal::derive(move || {
        let text = markdown.get();
        let count = text.split('\n').count();
        count.max(1)
    });

    view! {
        <div class="line-counter py-5">
            <div style=move || format!("transform: translateY(-{:.2}px);", scroll.get())>
                <For
                    each=move || 1..=line_count.get()
                    key=|line| *line
                    children=|line| {
                        view! {
                            <div class="line-counter-text">
                                { line }
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
