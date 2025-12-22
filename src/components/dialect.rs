use leptos::prelude::*;

use crate::api::parser::Dialect;
use crate::components::{icons::Icon, select::Select};

#[component]
pub fn DialectSelect(parser: RwSignal<Dialect>) -> impl IntoView {
    view! {
        <Select
            icon=Icon::Markdown
            options=vec![String::from("Common"), String::from("GitHub")]
            prop_value=Signal::derive(move || parser.get().to_string())
            on_change=move |value: String| {
                parser.set(value.parse().unwrap_or(Dialect::Common));
            }
        />
    }
}
