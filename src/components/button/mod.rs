use leptos::prelude::*;

pub mod copy;
pub mod delete;
pub mod download;
pub mod format_tables;

use crate::components::icons::Icon;
use crate::components::{Color, Size, State};

#[component]
pub fn Button<F>(
    #[prop(default=Signal::from(None), into)] text: Signal<Option<String>>,
    #[prop(optional, into)] icon: Option<Signal<Icon>>,
    #[prop(into, default=Signal::from(Color::Primary))] color: Signal<Color>,
    #[prop(into, default=Signal::from(Size::Normal))] size: Signal<Size>,
    #[prop(into, optional)] state: Option<Signal<State>>,
    #[prop(into, default=Signal::from(false))] is_rounded: Signal<bool>,
    #[prop(into, default=Signal::from(false))] has_smaller_padding: Signal<bool>,
    #[prop(into, default=Signal::from(false))] is_full_size: Signal<bool>,
    on_click: F,
) -> impl IntoView  
where F: Fn() + 'static {

    // let color = move || {
    //     color
    //         .map(|c| c.get())
    //         .unwrap_or(Color::Primary)
    // };

    let button_class = move || format!(
        "button {} {} {} {} {} {}", 
        color.get().to_class(), 
        size.get().to_class(), 
        state.unwrap_or_else(|| Signal::from(State::Normal)).read().to_class(),
        if is_rounded.get() { "is-rounded" } else { "" },
        if has_smaller_padding.get() { "px-2" } else { "" },
        if is_full_size.get() { "is-full-size" } else { "" },
    );

    view! {
        <button 
            class=button_class on:click=move |_| on_click()
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
