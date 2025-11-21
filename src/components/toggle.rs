use leptos::prelude::*;

use crate::components::icons::Icon;

#[allow(dead_code)]
#[component]
pub fn Toggle<F>(
    #[prop(optional, into)] text: Option<Signal<String>>,
    is_active: Signal<bool>,
    #[prop(into, default=Signal::from(false))] is_loading: Signal<bool>,
    on_action: F,
) -> impl IntoView
where F: Fn() + 'static {

    view! {
        <div class="is-flex is-align-items-center is-justify-content-center" class:is-disabled=move || is_loading.get()>
            <label class="toggle"
                class:active = move || is_active.get()
                on:click=move |_| on_action()
                
            >
                <span class="bubble">
                    <i 
                        class=Icon::Loading.as_fontawesome() 
                        class:is-hidden=move || !is_loading.get()
                    />
                </span>
            </label>
            {
                text.map(|text| {
                    view! {
                        <span class="pl-2">{ text }</span>
                    }
                })
            }
        </div>
    }
}
