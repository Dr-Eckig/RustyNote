use leptos::prelude::*;

#[allow(dead_code)]
pub enum TooltipDirection {
    Left,
    Right,
    Up,
    UpRight,
}

impl TooltipDirection {
    fn as_class(&self) -> &'static str {
        match self {
            TooltipDirection::Left => "is-left",
            TooltipDirection::Right => "is-right",
            TooltipDirection::Up => "is-up",
            TooltipDirection::UpRight => "is-right is-up",
        }
    }
}

#[component]
pub fn Tooltip(
    #[prop(into)] text: Signal<String>,
    #[prop(into, default=Signal::from(TooltipDirection::Left))] direction: Signal<TooltipDirection>,
    #[prop(default=Signal::from(false))] is_hidden: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let is_hidden = move || is_hidden.get() || text.get().is_empty();

    view! {
        <div class=move || format!("tooltip {}", direction.with(TooltipDirection::as_class))>
            <div class="tooltip-trigger">
                { children() }
            </div>
            <div class="tooltip-container" style=move || if is_hidden() { "display: none" } else { "" }>
                <div class="tooltip-content p-0">
                    <div class="tooltip-item is-size-7 p-2">
                        { text }
                    </div>
                </div>
            </div>
        </div>
    }
}
