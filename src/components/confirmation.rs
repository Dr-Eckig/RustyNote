use leptos::{html::Div, prelude::*};
use leptos_use::on_click_outside;

use crate::components::{Color, Size, button::Button, icons::Icon};

#[component]
pub fn Confirmation(
    #[prop(into)] confirmation_text: Signal<String>,
    on_confirmation: impl Fn() + 'static
) -> impl IntoView {

    let dropdown_visible = RwSignal::new(false);

    let dropdown_area = NodeRef::<Div>::new();
    let _ = on_click_outside(dropdown_area, move |_| {
        dropdown_visible.set(false)
    });

    view! {
        <div class="dropdown is-right" class:is-active=move || dropdown_visible.get()>
            <div class="dropdown-trigger">
                <Button 
                    aria_label=String::from("Delete")
                    icon=Icon::Delete 
                    size=Size::Small 
                    color=Color::Danger
                    on_click=move || dropdown_visible.set(!dropdown_visible.get())
                />
            </div>
            <div node_ref=dropdown_area class="dropdown-menu">
                <div class="dropdown-content">
                    <div class="dropdown-item">
                        { confirmation_text }
                    </div>
                    <div class="is-flex dropdown-item is-justify-content-space-between is-align-items-center">
                        <Button
                            aria_label=String::from("Confirm Deletion")
                            text="Delete"
                            icon=Icon::Delete 
                            size=Size::Small 
                            color=Color::Danger
                            on_click=move || {
                                on_confirmation();
                                dropdown_visible.set(!dropdown_visible.get());
                            }
                        />
                        <Button 
                            aria_label=String::from("Cancel")
                            text="Cancel"
                            icon=Icon::Cross 
                            size=Size::Small 
                            color=Color::Light
                            on_click=move || dropdown_visible.set(!dropdown_visible.get())
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}