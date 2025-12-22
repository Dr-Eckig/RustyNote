use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;

use crate::Mode;
use crate::api::markdown_formatter::handle_enter_for_lists;
use crate::api::parser::Dialect;
use crate::page::editor::controls::mobile::format_buttons::MobileSidebar;

#[component]
pub fn WriteSection(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
    mode: RwSignal<Mode>,
    mobile_sidebar_open: RwSignal<bool>,
) -> impl IntoView {
    let textarea_ref: NodeRef<html::Textarea> = NodeRef::new();
    let scroll = RwSignal::new(0.0);

    let sync_scroll_to_caret = move || {
        if let Some(textarea) = textarea_ref.get() {
            let len = textarea.value().len() as u32;
            let caret_at_end = textarea
                .selection_end()
                .ok()
                .flatten()
                .map(|pos| pos >= len)
                .unwrap_or(false);

            if caret_at_end {
                textarea.set_scroll_top(textarea.scroll_height());
            }

            scroll.set(textarea.scroll_top().into());
        }
    };

    view! {
        <MobileSidebar markdown parser sidebar_open=mobile_sidebar_open />
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
                class="textarea has-fixed-size card is-family-monospace full-height p-5"
                placeholder="Write your Markdown here..."
                node_ref=textarea_ref
                prop:value=markdown
                on:input=move |ev| {
                    markdown.set(event_target_value(&ev));
                    sync_scroll_to_caret();
                }
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        markdown.set(handle_enter_for_lists());
                        if let Some(textarea) = textarea_ref.get() {
                            markdown.set(textarea.value());
                            scroll.set(textarea.scroll_top().into());
                        }
                        sync_scroll_to_caret();
                    }
                }
                on:scroll=move |ev| {
                    if let Some(target) = ev.target()
                        && let Ok(textarea) =
                            target.dyn_into::<web_sys::HtmlTextAreaElement>()
                    {
                        scroll.set(textarea.scroll_top().into());
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn LineColumn(markdown: RwSignal<String>, scroll: ReadSignal<f64>) -> impl IntoView {
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
