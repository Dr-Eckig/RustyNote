use leptos::prelude::*;
use leptos_use::use_media_query;

use crate::components::icons::Icon;

#[derive(Clone)]
pub struct Tab {
    pub name: String,
    pub icon: Icon,
}

#[component]
pub fn Tabs(active_tab: RwSignal<usize>, #[prop(into)] tabs: Signal<Vec<Tab>>) -> impl IntoView {
    let is_mobile = use_media_query("(max-width: 768px)");
    let show_text = Signal::derive(move || !is_mobile.get());

    view! {
        <div class="tabs is-toggle is-fullwidth">
            <ul>
                <For
                    each=move || tabs.get().into_iter().enumerate()
                    key=|(_, tab)| tab.name.clone()
                    children=move |(index, tab)| {
                        let is_active = move || active_tab.get() == index;

                        view! {
                            <li class:is-active=is_active>
                                <a on:click=move |_| active_tab.set(index)>
                                    <span class="icon is-small">
                                        <i class=tab.icon.as_fontawesome() aria-hidden="true"></i>
                                    </span>
                                    <Show when=move || show_text.get()>
                                        <span>{tab.name.clone()}</span>
                                    </Show>
                                </a>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
