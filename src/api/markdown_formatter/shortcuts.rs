use leptos::{
    ev::keydown,
    prelude::{RwSignal, Set},
    *,
};
use leptos_use::{use_document, use_event_listener};
use web_sys::KeyboardEvent;

use crate::api::markdown_formatter::format::TextFormattingType;

/// Sets up keyboard shortcuts for markdown formatting.
///
/// This function registers global keyboard event listeners that apply markdown
/// formatting when the corresponding key combinations are pressed.
///
/// # Shortcuts
///
/// - `Ctrl+B`: Bold text (`**text**`)
/// - `Ctrl+H`: Heading (cycles through heading levels)
/// - `Ctrl+#`: Code block (```)
/// - `Ctrl+M`: Monospace/inline code (`` `text` ``)
///
/// # Examples
///
/// ```rust,ignore
/// use markdown::api::markdown_formatter::setup_shortcuts;
/// use leptos::prelude::*;
///
/// // Requires a Leptos context and browser environment.
/// let markdown = RwSignal::new(String::new());
/// setup_shortcuts(markdown);
/// ```
pub fn setup_shortcuts(markdown: RwSignal<String>) {
    let _ = use_event_listener(use_document(), keydown, move |ev: KeyboardEvent| {
        for (shortcut, action) in SHORTCUTS {
            if shortcut.matches(&ev) {
                ev.prevent_default();
                match action {
                    ShortcutKey::Bold => markdown.set(
                        TextFormattingType::Inline {
                            prefix: "**",
                            suffix: "**",
                        }
                        .apply_text_formatting(),
                    ),
                    ShortcutKey::Heading => {
                        markdown.set(TextFormattingType::Heading.apply_text_formatting())
                    }
                    ShortcutKey::CodeBlock => {
                        markdown.set(TextFormattingType::CodeBlock.apply_text_formatting())
                    }
                    ShortcutKey::Monospace => markdown.set(
                        TextFormattingType::Inline {
                            prefix: "`",
                            suffix: "`",
                        }
                        .apply_text_formatting(),
                    ),
                }
                break;
            }
        }
    });
}

enum ShortcutKey {
    Bold,
    Heading,
    CodeBlock,
    Monospace,
}

#[derive(Clone)]
struct Shortcut {
    key: &'static str,
    ctrl: bool,
    alt: bool,
    shift: bool,
}

impl Shortcut {
    const fn new(key: &'static str, ctrl: bool, alt: bool, shift: bool) -> Self {
        Self {
            key,
            ctrl,
            alt,
            shift,
        }
    }

    fn matches(&self, event: &KeyboardEvent) -> bool {
        event.key() == self.key
            && event.ctrl_key() == self.ctrl
            && event.alt_key() == self.alt
            && event.shift_key() == self.shift
    }
}

const SHORTCUTS: &[(&Shortcut, ShortcutKey)] = &[
    (&Shortcut::new("b", true, false, false), ShortcutKey::Bold),
    (
        &Shortcut::new("h", true, false, false),
        ShortcutKey::Heading,
    ),
    (
        &Shortcut::new("#", true, false, false),
        ShortcutKey::CodeBlock,
    ),
    (
        &Shortcut::new("m", true, false, false),
        ShortcutKey::Monospace,
    ),
];
