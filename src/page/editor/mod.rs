use leptos::prelude::*;
use leptos_use::use_media_query;

use crate::api::markdown_formatter::setup_shortcuts;
use crate::Mode;
use crate::api::parser::Dialect;
use crate::page::editor::controls::Controls;
use crate::page::editor::read_section::ReadSection;
use crate::page::editor::write_section::WriteSection;

mod controls;
mod read_section;
mod write_section;

#[component]
pub fn MarkdownEditor(
    markdown: RwSignal<String>,
    mode: RwSignal<Mode>,
    parser: RwSignal<Dialect>,
) -> impl IntoView {

    setup_shortcuts(markdown);

    let mobile_sidebar_open = RwSignal::new(false);

    Effect::new(move || {
        let mode = mode.get();

        if matches!(mode, Mode::Read) {
            mobile_sidebar_open.set(false);
        }
    });

    let is_mobile = use_media_query("(max-width: 768px)");

    view! {
        <Controls markdown parser sidebar_open=mobile_sidebar_open mode />

        <main 
            class="columns is-mobile is-flex-grow-1 px-0 m-0" style="overflow: hidden;"
            class:px-5=move || is_mobile.get()    
        >
            <div 
                class="column is-narrow is-hidden-mobile"
                class:is-hidden=move || matches!(mode.get(), Mode::Write | Mode::Split)
            >
                <div class="line-counter" />
            </div>

            { move || (mode.get() == Mode::Write || mode.get() == Mode::Split).then(|| 
                view! {
                    <WriteSection markdown parser mode mobile_sidebar_open />
                }
            )}

            { move || (mode.get() == Mode::Read || mode.get() == Mode::Split).then(|| 
                view! {
                    <ReadSection markdown=markdown.read_only() parser=parser.read_only() mode=mode.read_only() />
                }
            )}

            <div class="column is-narrow is-hidden-mobile">
                <div class="line-counter" />
            </div>
        </main>
    }
}