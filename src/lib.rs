use leptoaster::{provide_toaster, Toaster};
use leptos::{prelude::*};
use crate::{api::{local_storage::use_persistent_signal, parser::Dialect}, page::{editor::MarkdownEditor, header::Header}};

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


    view! {
        <div class="is-flex is-flex-direction-column page-height page-background-color">
            <Toaster />
            <Header markdown mode parser />
            <MarkdownEditor mode markdown parser /> 
        </div>
    }
}
