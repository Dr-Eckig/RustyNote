use leptos::{html::Div, prelude::*};
use leptos_use::on_click_outside;

use crate::components::{Color, Size, button::Button, icons::Icon};

pub enum DropdownDirection {
    Left,
    Right
}

impl DropdownDirection {
    fn as_class(&self) -> &'static str {
        match self {
            DropdownDirection::Left => "is-left",
            DropdownDirection::Right => "is-right"
        }
    }
}

#[component]
pub fn DropdownButton(
    aria_label: String,
    #[prop(into)] icon: Signal<Icon>, 
    #[prop(default=DropdownDirection::Left)] direction: DropdownDirection,
    children: Children
) -> impl IntoView {

    let dropdown_visible = RwSignal::new(false);

    let area = NodeRef::<Div>::new();
    let _ = on_click_outside(area, move |_| {
        dropdown_visible.set(false)
    });

    view! {
        <div class=format!("dropdown is-hoverable {}", direction.as_class()) class:is-active=move || dropdown_visible.get()>
            <div class="dropdown-trigger">
                <Button
                    aria_label
                    icon
                    size=Size::Normal
                    color=Color::Transparent
                    on_click=move || dropdown_visible.set(!dropdown_visible.get())
                />
            </div>

            <div node_ref=area class="dropdown-menu" role="menu">
                <div class="dropdown-content">
                    { children() }
                </div>
            </div>
        </div>
    }
}