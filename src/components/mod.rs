pub mod icons;
pub mod button;
pub mod select;
pub mod confirmation;
pub mod logo;
pub mod tooltip;
pub mod tabs;
pub mod theme;
pub mod dropdown;
pub mod mobile;
mod doc;

pub use doc::modal::HelpModal;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Color {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Light,
    White,
    Dark,
    Transparent,
}

impl Color {
    pub fn to_class(self) -> &'static str {
        match self {
            Color::Primary => "is-primary",
            Color::Link => "is-link",
            Color::Info => "is-info",
            Color::Success => "is-success",
            Color::Warning => "is-warning",
            Color::Danger => "is-danger",
            Color::Light => "is-light",
            Color::White => "is-white",
            Color::Dark => "is-dark",
            Color::Transparent => "is-transparent",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub fn to_class(self) -> &'static str {
        match self {
            Size::Small => "is-small",
            Size::Normal => "is-normal",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Active,
    Disabled,
    Normal,
}

impl State {
    pub fn to_class(self) -> &'static str {
        match self {
            State::Normal => "",
            State::Active => "is-active",
            State::Disabled => "",
        }
    }
}
