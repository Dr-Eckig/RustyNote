use leptos::{html::Div, prelude::*};
use leptos_use::on_click_outside;

use crate::components::{Color, Size, button::Button, doc::{commonmark::CommonMarkDoc, gfm::GitHubExtensionDoc, rustynote::RustyNoteDoc}, icons::Icon, tabs::{Tab, Tabs}};

#[component]
pub fn HelpModal(
    #[prop(into, default=Signal::from(false))] is_dropdown_item: Signal<bool>,
    #[prop(default=false)] fullsize_button: bool,
) -> impl IntoView {

    let active_tab = RwSignal::new(0);
    let modal_visible = RwSignal::new(false);

    let modal_area = NodeRef::<Div>::new();
    let _ = on_click_outside(modal_area, move |_| {
        modal_visible.set(false)
    });

    view! {
        <Button
            aria_label=String::from("Open Documentation")
            text="Documentation"
            icon=Icon::Help
            color=Signal::derive(move || if is_dropdown_item.get() { Color::Transparent } else { Color::None })
            size=Size::Normal
            has_smaller_padding=is_dropdown_item
            is_full_size=fullsize_button
            on_click=move || modal_visible.set(true)
        />
        <div class="modal" class:is-active=move || modal_visible.get()>
            <div class="modal-background" />
            <div class="modal-card container" node_ref=modal_area>
                <header class="modal-card-head">
                    <p class="modal-card-title">Documentation</p>
                    <button class="delete" aria-label="close" on:click=move |_| modal_visible.set(false) />
                </header>
                <section class="modal-card-body px-5">
                    {
                        move || {
                            match active_tab.get() {
                                0 => view! { <RustyNoteDoc /> }.into_any(),
                                1 => view! { <CommonMarkDoc /> }.into_any(),
                                _ => view! { <GitHubExtensionDoc /> }.into_any(),
                            }
                        }
                    }
                </section>
                <footer class="modal-card-foot">
                    <Tabs 
                        active_tab=active_tab
                        tabs=vec![
                            Tab { name: String::from("RustyNote"), icon: Icon::Info },
                            Tab { name: String::from("CommonMark"), icon: Icon::Markdown },
                            Tab { name: String::from("GitHub Extension (GFM)"), icon: Icon::GitHub },
                        ]
                    />
                </footer>
            </div>
        </div>
    }
}
