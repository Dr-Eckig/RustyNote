use leptos::prelude::*;
use leptos_use::use_document;

use crate::{api::local_storage::use_persistent_signal, components::{Color, Size, button::Button, icons::Icon}};

#[component] 
pub fn ThemeButton(
    #[prop(into, default=Signal::from(false))] show_text: Signal<bool>,
    #[prop(default=false)] fullsize_button: bool
) -> impl IntoView {

    let is_dark_theme = use_persistent_signal(String::from("theme"));

    Effect::new(move ||{
        let is_dark_theme = is_dark_theme.get();

        let theme = if is_dark_theme { "dark" } else { "light" };
        let document = use_document();
        let html = document.document_element().unwrap();
        html.set_attribute("data-theme", theme).unwrap();
    });

    view! {
        <Button
            text=Signal::derive(move || if show_text.get() { Some(String::from("Switch Theme")) } else { None })
            icon=Signal::derive(move || if is_dark_theme.get() { Icon::Sun } else { Icon::Moon }) 
            color=Color::Transparent
            size=Size::Normal
            is_rounded=true
            has_smaller_padding=true
            is_full_size=fullsize_button
            on_click=move || is_dark_theme.set(!is_dark_theme.get())
        />
    }
}