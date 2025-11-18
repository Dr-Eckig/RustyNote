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
    children: Children
) -> impl IntoView {

    view! {
        <div class=move || format!("tooltip {}", direction.with(TooltipDirection::as_class))>
            <div class="tooltip-trigger">
                { children() }
            </div>
            <div class="tooltip-container">
                <div class="tooltip-content p-0">
                    <div class="tooltip-item is-size-7 p-2">
                        { text }
                    </div>
                </div>
            </div>
        </div>
    }
}