use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;

use crate::api::markdown_formatter::combine_text_slices;

fn get_textarea() -> HtmlTextAreaElement {
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("markdown-textarea"))
        .and_then(|el| el.dyn_into::<HtmlTextAreaElement>().ok())
        .expect("Couldn't get textarea")
}

/// Represents the current textarea selection including the full text and cursor indices.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "Hello world".into(),
///     selected_text: Some("Hello".into()),
///     before_selection: String::new(),
///     after_selection: " world".into(),
///     start_index: 0,
///     end_index: 5,
/// };
///
/// assert_eq!(selection.inner_text(), "Hello");
/// ```
#[derive(Debug, Clone)]
pub struct Selection {
    pub textarea_value: String,
    pub selected_text: Option<String>,
    pub before_selection: String,
    pub after_selection: String,
    pub start_index: usize,
    pub end_index: usize,
}

impl Selection {
    pub(crate) fn new(textarea_value: String, start: usize, end: usize) -> Self {
        let selected_text = if end == start {
            None
        } else {
            Some(textarea_value[start..end].to_string())
        };

        let before_selection = textarea_value[..start].to_string();
        let after_selection = textarea_value[end..].to_string();

        Selection {
            textarea_value,
            selected_text,
            before_selection,
            after_selection,
            start_index: start,
            end_index: end,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_caret_position(textarea_value: String, caret_position: usize) -> Self {
        let before_selection = textarea_value[..caret_position].to_string();
        let after_selection = textarea_value[caret_position..].to_string();

        Selection {
            textarea_value,
            selected_text: None,
            before_selection,
            after_selection,
            start_index: caret_position,
            end_index: caret_position,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_with_text(textarea_value: String, selected_text: Option<String>) -> Self {
        let (start_index, end_index) = if let Some(ref sel) = selected_text {
            if let Some(pos) = textarea_value.find(sel) {
                (pos, pos + sel.len())
            } else {
                (0, 0)
            }
        } else {
            let len = textarea_value.len();
            (len, len)
        };

        let before_selection = textarea_value[..start_index].to_string();
        let after_selection = textarea_value[end_index..].to_string();

        Selection {
            textarea_value,
            selected_text,
            before_selection,
            after_selection,
            start_index,
            end_index,
        }
    }

    /// Returns the currently selected text or an empty string when nothing is selected.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "Hello world".into(),
    /// #     selected_text: Some("Hello".into()),
    /// #     before_selection: String::new(),
    /// #     after_selection: " world".into(),
    /// #     start_index: 0,
    /// #     end_index: 5,
    /// # };
    /// assert_eq!(selection.inner_text(), "Hello");
    /// ```
    pub fn inner_text(&self) -> &str {
        self.selected_text.as_deref().unwrap_or("")
    }

    /// Returns the portion of the text that appears before the selection.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "Hello world".into(),
    /// #     selected_text: Some("Hello".into()),
    /// #     before_selection: String::new(),
    /// #     after_selection: " world".into(),
    /// #     start_index: 0,
    /// #     end_index: 5,
    /// # };
    /// assert_eq!(selection.before(), "");
    /// ```
    pub fn before(&self) -> &str {
        &self.before_selection
    }

    /// Returns the text that appears after the selection.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "Hello world".into(),
    /// #     selected_text: Some("Hello".into()),
    /// #     before_selection: String::new(),
    /// #     after_selection: " world".into(),
    /// #     start_index: 0,
    /// #     end_index: 5,
    /// # };
    /// assert_eq!(selection.after(), " world");
    /// ```
    pub fn after(&self) -> &str {
        &self.after_selection
    }

    /// Indicates whether the selection is empty (caret only).
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let cursor_only = Selection {
    /// #     textarea_value: "Hello".into(),
    /// #     selected_text: None,
    /// #     before_selection: "Hello".into(),
    /// #     after_selection: String::new(),
    /// #     start_index: 5,
    /// #     end_index: 5,
    /// # };
    /// assert!(cursor_only.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start_index == self.end_index
    }

    /// Returns `true` when the selection spans multiple lines.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "line one\nline two".into(),
    /// #     selected_text: Some("line one\nline two".into()),
    /// #     before_selection: String::new(),
    /// #     after_selection: String::new(),
    /// #     start_index: 0,
    /// #     end_index: 17,
    /// # };
    /// assert!(selection.is_multiline());
    /// ```
    pub fn is_multiline(&self) -> bool {
        self.inner_text().contains('\n')
    }

    /// Returns the line that currently contains the caret.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "first\nsecond".into(),
    /// #     selected_text: None,
    /// #     before_selection: "first\n".into(),
    /// #     after_selection: "second".into(),
    /// #     start_index: 6,
    /// #     end_index: 6,
    /// # };
    /// assert_eq!(selection.current_line(), "second");
    /// ```
    pub fn current_line(&self) -> &str {
        let start = crate::api::markdown_formatter::textarea::line_start_at(
            &self.textarea_value,
            self.start_index,
        );
        let end = crate::api::markdown_formatter::textarea::line_end_at(
            &self.textarea_value,
            self.start_index,
        );
        &self.textarea_value[start..end]
    }

    /// Returns the byte bounds of the selected lines.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "first\nsecond".into(),
    /// #     selected_text: Some("second".into()),
    /// #     before_selection: "first\n".into(),
    /// #     after_selection: String::new(),
    /// #     start_index: 6,
    /// #     end_index: 12,
    /// # };
    /// assert_eq!(selection.line_bounds(), (6, 12));
    /// ```
    pub fn line_bounds(&self) -> (usize, usize) {
        let text = &self.textarea_value;
        let start = crate::api::markdown_formatter::textarea::line_start_at(text, self.start_index);
        let end = if self.end_index == text.len() {
            text.len()
        } else {
            crate::api::markdown_formatter::textarea::line_end_at(text, self.end_index)
        };
        (start, end)
    }

    /// Splits the selected block into individual lines while preserving empty trailing lines.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "row 1\nrow 2".into(),
    /// #     selected_text: Some("row 1\nrow 2".into()),
    /// #     before_selection: String::new(),
    /// #     after_selection: String::new(),
    /// #     start_index: 0,
    /// #     end_index: 10,
    /// # };
    /// assert_eq!(selection.selected_lines(), vec!["row 1", "row 2"]);
    /// ```
    pub fn selected_lines(&self) -> Vec<&str> {
        let (start, end) = self.line_bounds();
        let slice = &self.textarea_value[start..end];
        let mut lines: Vec<&str> = slice.split('\n').collect();
        if lines.is_empty() {
            lines.push("");
        }
        lines
    }

    /// Replaces the byte range inside the textarea text and returns the updated string.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "Hello world".into(),
    /// #     selected_text: Some("world".into()),
    /// #     before_selection: "Hello ".into(),
    /// #     after_selection: String::new(),
    /// #     start_index: 6,
    /// #     end_index: 11,
    /// # };
    /// let replaced = selection.replace_range(6, 11, "Markdown");
    /// assert_eq!(replaced, "Hello Markdown");
    /// ```
    pub fn replace_range(&self, start: usize, end: usize, replacement: &str) -> String {
        let string_capacity = self.textarea_value.len() + replacement.len();

        combine_text_slices(
            vec![
                &self.textarea_value[..start],
                replacement,
                &self.textarea_value[end..],
            ],
            string_capacity,
        )
    }

    /// Counts how many newline characters appear before the provided byte index.
    ///
    /// ```rust
    /// # use markdown::api::markdown_formatter::textarea::Selection;
    /// # let selection = Selection {
    /// #     textarea_value: "a\nb\n c".into(),
    /// #     selected_text: None,
    /// #     before_selection: String::new(),
    /// #     after_selection: String::new(),
    /// #     start_index: 0,
    /// #     end_index: 0,
    /// # };
    /// assert_eq!(selection.line_index_of(3), 1);
    /// ```
    pub fn line_index_of(&self, pos: usize) -> usize {
        self.textarea_value[..pos].matches('\n').count()
    }
}

/// Reads the selection state from the active textarea element.
///
/// Note: Browser APIs return character positions, not byte positions.
/// This function converts them to byte positions for safe string operations.
///
/// ```rust,ignore
/// use markdown::api::markdown_formatter::textarea::get_current_selection;
///
/// // Requires a browser environment.
/// let selection = get_current_selection();
/// println!("{}", selection.textarea_value);
/// ```
pub(crate) fn get_current_selection() -> Selection {
    let textarea = get_textarea();
    let textarea_value = textarea.value();

    // Browser APIs return character positions, convert to byte positions
    let start_char_pos = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
    let end_char_pos = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start_char_pos as u32) as usize;

    let start_byte_pos = char_to_byte_pos_safe(&textarea_value, start_char_pos);
    let end_byte_pos = char_to_byte_pos_safe(&textarea_value, end_char_pos);

    Selection::new(textarea_value, start_byte_pos, end_byte_pos)
}

/// Updates the textarea contents and caret bounds.
///
/// Note: This function expects byte positions but converts them to character positions
/// before sending to the browser, as browser APIs expect character positions.
///
/// ```rust,ignore
/// use markdown::api::markdown_formatter::textarea::set_cursor;
///
/// // Requires a browser environment.
/// set_cursor("Hello".into(), 0, 0);
/// ```
pub(crate) fn set_cursor(
    new_value: String,
    new_sel_start_byte: u32,
    new_sel_end_byte: u32,
) -> String {
    let textarea = get_textarea();

    textarea.set_value(&new_value);

    // Convert byte positions to character positions for browser APIs
    let new_sel_start_char = byte_to_char_pos(&new_value, new_sel_start_byte as usize) as u32;
    let new_sel_end_char = byte_to_char_pos(&new_value, new_sel_end_byte as usize) as u32;

    // Clamp to valid character positions
    let max_char_pos = new_value.chars().count() as u32;
    let clamped_start = new_sel_start_char.min(max_char_pos);
    let clamped_end = new_sel_end_char.min(max_char_pos);

    textarea.set_selection_start(Some(clamped_start)).ok();
    textarea.set_selection_end(Some(clamped_end)).ok();
    textarea.focus().ok();

    new_value
}

/// Calculates the byte index of the start of the line containing `idx`.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::line_start_at;
///
/// assert_eq!(line_start_at("a\nb", 2), 2);
/// ```
pub fn line_start_at(value: &str, idx: usize) -> usize {
    value[..idx].rfind('\n').map(|p| p + 1).unwrap_or(0)
}

/// Calculates the byte index of the end of the line containing `idx`.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::line_end_at;
///
/// assert_eq!(line_end_at("a\nb", 0), 1);
/// ```
pub fn line_end_at(value: &str, idx: usize) -> usize {
    value[idx..]
        .find('\n')
        .map(|p| idx + p)
        .unwrap_or(value.len())
}

/// Converts a character position to a byte position in a UTF-8 string.
/// This is necessary because browser APIs return character positions,
/// but Rust string slicing requires byte positions.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::char_to_byte_pos;
///
/// assert_eq!(char_to_byte_pos("Ä", 0), 0);
/// assert_eq!(char_to_byte_pos("Ä", 1), 2); // "Ä" is 2 bytes
/// assert_eq!(char_to_byte_pos("Hello", 5), 5);
/// ```
pub fn char_to_byte_pos(text: &str, char_pos: usize) -> usize {
    text.char_indices()
        .nth(char_pos)
        .map(|(byte_pos, _)| byte_pos)
        .unwrap_or_else(|| text.len())
}

/// Safely converts a character position to a byte position, clamping to valid bounds.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::char_to_byte_pos_safe;
///
/// assert_eq!(char_to_byte_pos_safe("Ä", 5), 2); // Clamped to end
/// ```
pub fn char_to_byte_pos_safe(text: &str, char_pos: usize) -> usize {
    let byte_pos = char_to_byte_pos(text, char_pos);
    byte_pos.min(text.len())
}

/// Converts a byte position to a character position in a UTF-8 string.
/// This is necessary when sending positions back to browser APIs,
/// which expect character positions.
///
/// ```rust
/// use markdown::api::markdown_formatter::textarea::byte_to_char_pos;
///
/// assert_eq!(byte_to_char_pos("Ä", 0), 0);
/// assert_eq!(byte_to_char_pos("Ä", 2), 1); // "Ä" is 2 bytes
/// assert_eq!(byte_to_char_pos("Hello", 5), 5);
/// ```
pub fn byte_to_char_pos(text: &str, byte_pos: usize) -> usize {
    let clamped_byte_pos = byte_pos.min(text.len());
    text[..clamped_byte_pos].chars().count()
}
