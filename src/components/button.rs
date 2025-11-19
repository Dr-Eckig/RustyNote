use leptos::prelude::*;

use crate::components::icons::Icon;
use crate::components::{Color, Size, State};

#[component]
pub fn Button<F>(
    #[prop(default=Signal::from(None), into)] text: Signal<Option<String>>,
    #[prop(optional, into)] icon: Option<Signal<Icon>>,
    #[prop(into, optional)] color: Option<Signal<Color>>,
    #[prop(default = Size::Normal)] size: Size,
    #[prop(into, optional)] state: Option<Signal<State>>,
    #[prop(optional)] is_rounded: bool,
    #[prop(optional)] has_smaller_padding: bool,
    on_click: F,
) -> impl IntoView  
where F: Fn() + 'static {

    let color = move || {
        color
            .map(|c| c.get())
            .unwrap_or(Color::Primary)
    };

    let button_class = move || format!(
        "button {} {} {} {} {}", 
        color().to_class(), 
        size.to_class(), 
        state.unwrap_or_else(|| Signal::from(State::Normal)).read().to_class(),
        if is_rounded { "is-rounded" } else { "" },
        if has_smaller_padding { "px-2" } else { "" },
    );

    view! {
        <button 
            class=move || button_class() on:click=move |_| on_click()
            disabled=move || state.map(|s| s.get() == State::Disabled).unwrap_or(false)
        >
            {
                icon.map(|icon_signal| {
                    view! { 
                        <span class="icon" class:m-0=move || text.get().is_none() class:px-2=move || text.get().is_some()>
                            <i class=move || icon_signal.get().as_fontawesome() />
                        </span> 
                    }
                })
            } 
            <span>{ text }</span>
        </button>
    }
}
