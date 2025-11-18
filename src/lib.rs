use leptoaster::{provide_toaster, Toaster};
use leptos::{prelude::*};
use crate::{api::{local_storage::use_persistent_signal, parser::Dialect}, page::{controls::{Controls, MobileControls}, editor::MarkdownEditor, header::Header}};

mod page;
mod components;
pub mod api;

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Write,
    Split,
    Read
}

#[component]
pub fn App() -> impl IntoView {

    let mode = RwSignal::new(Mode::Split); 
    let markdown: RwSignal<String> = use_persistent_signal("markdown_content".to_string());

    let parser: RwSignal<Dialect> = use_persistent_signal("markdown_parser".to_string());

    provide_toaster();

    let mobile_sidebar_open = RwSignal::new(false);

    view! {
        <div class="is-flex is-flex-direction-column page-height page-background-color">
            <Toaster />
            <Header markdown mode parser />
            <Controls markdown parser />
            <MobileControls markdown sidebar_open=mobile_sidebar_open /> 
            <MarkdownEditor mode markdown parser mobile_sidebar_open /> 
        </div>
    }
}
