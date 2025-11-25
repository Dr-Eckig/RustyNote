use leptos::prelude::*;
use crate::components::icons::Icon;

#[component]
pub fn Select(
    options: Vec<String>,
    #[prop(into, optional)] icon: Option<Signal<Icon>>,
    #[prop(into)] prop_value: Signal<String>,
    on_change: impl Fn(String) + 'static,
) -> impl IntoView {
    
    view! {
        <div class="control" class:has-icons-left=move || icon.is_some()>
            <div class="select is-full-size">
                <select
                    class="is-full-size"
                    prop:value=move || prop_value.get()
                    on:change=move |ev| {
                        on_change(event_target_value(&ev));
                    }
                >
                    <For
                        each=move || options.clone()
                        key=|o| o.clone()
                        children=|o| {
                            view! {
                                <option>{ o }</option>
                            }
                        }
                    />
                </select>
            </div>
            {
                icon.map(|icon| { move || 
                    view! {
                        <span class="icon is-left">
                            <i class=icon.get().as_fontawesome() />
                        </span>
                    }
                })
            }
        </div>
    }
}