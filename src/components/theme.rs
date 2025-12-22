use std::{fmt, str::FromStr};

use crate::{
    api::local_storage::use_persistent_signal,
    components::{icons::Icon, select::Select},
};
use leptos::prelude::*;
use leptos_use::{use_document, use_preferred_dark};

#[derive(PartialEq, Clone, Debug, Default)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Light => "Light",
                Theme::Dark => "Dark",
                Theme::System => "System",
            }
        )
    }
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            "system" => Ok(Theme::System),
            _ => Err(()),
        }
    }
}

#[component]
pub fn ThemeSelect() -> impl IntoView {
    let theme: RwSignal<Theme> = use_persistent_signal(String::from("theme"));
    let dark_preferred = use_preferred_dark();

    Effect::new(move || {
        let theme = theme.get();

        let is_dark = match theme {
            Theme::Light => false,
            Theme::Dark => true,
            Theme::System => dark_preferred.get(),
        };

        let data_theme = if is_dark { "dark" } else { "light" };
        let document = use_document();
        let html = document.document_element().unwrap();
        html.set_attribute("data-theme", data_theme).unwrap();
    });

    let icon = Signal::derive(move || match theme.get() {
        Theme::Light => Icon::Sun,
        Theme::Dark => Icon::Moon,
        Theme::System => {
            if dark_preferred.get() {
                Icon::Moon
            } else {
                Icon::Sun
            }
        }
    });

    view! {
        <Select
            icon
            options=vec![String::from("Light"), String::from("Dark"), String::from("System")]
            prop_value=Signal::derive(move || theme.get().to_string())
            on_change=move |value: String| {
                theme.set(value.parse().unwrap());
            }
        />
    }
}
