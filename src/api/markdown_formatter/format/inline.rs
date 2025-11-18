use crate::api::markdown_formatter::textarea::Selection;
use super::SelectionFormatter;

/// Formatter that toggles inline markdown markers such as `**` or `_`.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{Inline, SelectionFormatter};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "hello".into(),
///     selected_text: Some("hello".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 5,
/// };
///
/// let (text, start, end) = Inline::new(&selection, "**", "**").format();
/// assert_eq!(text, "**hello**");
/// assert_eq!((start, end), (2, 7));
/// ```
pub struct Inline<'a> {
    pub prefix: &'static str,
    pub suffix: &'static str,
    pub selection: &'a Selection,
}

impl<'a> Inline<'a> {

    /// Creates a formatter that wraps the selection with the given prefix and suffix.
    pub fn new(selection: &'a Selection, prefix: &'static str, suffix: &'static str) -> Inline<'a> {
        Inline {
            prefix,
            suffix,
            selection,
        }
    }

    fn apply_inline_formatting(&self) -> (String, u32, u32) {
        if !self.selection.is_multiline() {
            self.apply_word_mode()
        } else {
            self.apply_line_mode()
        }
    }

    fn apply_word_mode(&self) -> (String, u32, u32) {
        let (mut start, mut end) = if self.selection.selected_text.is_none() {
            self.get_word_bounds_at_cursor()
        } else {
            (self.selection.start_index, self.selection.end_index)
        };

        while end > start && self.selection.textarea_value.as_bytes()[end - 1].is_ascii_whitespace() {
            end -= 1;
        }

        let text = &self.selection.textarea_value;

        if self.selection.selected_text.is_some() {
            if let Some(ref selected) = self.selection.selected_text {
                if selected.starts_with(self.prefix) && selected.ends_with(self.suffix) {
                    return self.unwrap_full_selection();
                }
            }

            if start >= self.prefix.len()
                && end + self.suffix.len() <= text.len()
                && &text[start - self.prefix.len()..start] == self.prefix
                && &text[end..end + self.suffix.len()] == self.suffix
            {
                let outer_start = start - self.prefix.len();
                let outer_end = end + self.suffix.len();
                let new_text = format!(
                    "{}{}{}",
                    &text[..outer_start],
                    &text[start..end],
                    &text[outer_end..],
                );
                let new_start = outer_start as u32;
                let new_end = (outer_start + (end - start)) as u32;
                return (new_text, new_start, new_end);
            }
        }

        if start >= self.prefix.len() && &text[start - self.prefix.len()..start] == self.prefix {
            start -= self.prefix.len();
        }
        if end + self.suffix.len() <= text.len() && &text[end..end + self.suffix.len()] == self.suffix {
            end += self.suffix.len();
        }

        if self.selection.selected_text.is_none()
            && end > start
            && (end - start) >= (self.prefix.len() + self.suffix.len())
        {
            let segment = &text[start..end];
            if segment.starts_with(self.prefix) && segment.ends_with(self.suffix) {
                let inner_start = start + self.prefix.len();
                let inner_end = end - self.suffix.len();
                return self.unwrap_word(inner_start, inner_end);
            }
        }

        if self.is_wrapped(start, end) {
            self.unwrap_word(start, end)
        } else {
            self.wrap_word(start, end)
        }
    }

    fn unwrap_full_selection(&self) -> (String, u32, u32) {
        let t = &self.selection.textarea_value;
        let start = self.selection.start_index;
        let end = self.selection.end_index;

        let inner = &t[start + self.prefix.len() .. end - self.suffix.len()];

        let new_text = format!(
            "{}{}{}",
            &t[..start],
            inner,
            &t[end..],
        );

        let new_start = start as u32;
        let new_end = (start + inner.len()) as u32;
        (new_text, new_start, new_end)
    }

    fn get_word_bounds_at_cursor(&self) -> (usize, usize) {
        let text = &self.selection.textarea_value;
        let cursor = self.selection.start_index;

        let start = text[..cursor].rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let end = text[cursor..].find(|c: char| c.is_whitespace())
            .map(|i| cursor + i)
            .unwrap_or(text.len());

        (start, end)
    }

    fn is_wrapped(&self, start: usize, end: usize) -> bool {
        let t = &self.selection.textarea_value;
        start >= self.prefix.len()
            && end + self.suffix.len() <= t.len()
            && &t[start - self.prefix.len()..start] == self.prefix
            && &t[end..end + self.suffix.len()] == self.suffix
    }

    fn wrap_word(&self, start: usize, end: usize) -> (String, u32, u32) {
        let t = &self.selection.textarea_value;
        if self.selection.selected_text.is_some() {
            let new_text = format!(
                "{}{}{}{}{}",
                &t[..start],
                self.prefix,
                &t[start..end],
                self.suffix,
                &t[end..],
            );

            let new_start = (start + self.prefix.len()) as u32;
            let new_end = (new_start + (end - start) as u32) as u32;
            return (new_text, new_start, new_end);
        }

        let cursor = self.selection.start_index;
        let cursor_offset = cursor.saturating_sub(start);

        let new_text = format!(
            "{}{}{}{}{}",
            &t[..start],
            self.prefix,
            &t[start..end],
            self.suffix,
            &t[end..],
        );

        let caret = (start + self.prefix.len() + cursor_offset) as u32;
        (new_text, caret, caret)
    }

    fn unwrap_word(&self, start: usize, end: usize) -> (String, u32, u32) {
        let t = &self.selection.textarea_value;

        let real_start = start - self.prefix.len();
        let real_end = end + self.suffix.len();

        let new_text = format!(
            "{}{}{}",
            &t[..real_start],
            &t[start..end],
            &t[real_end..],
        );

        let cursor = self.selection.start_index;

        let caret = if cursor >= start && cursor <= end {
            (real_start + (cursor - start)) as u32
        } else {
            real_start as u32
        };

        (new_text, caret, caret)
    }

    fn apply_line_mode(&self) -> (String, u32, u32) {
        let sel = &self.selection;

        let (ls, le) = sel.line_bounds();
        let lines = sel.selected_lines();

        let already = self.all_lines_are_wrapped(&lines);
        let new_lines = if already {
            self.remove_inline(&lines)
        } else {
            self.add_inline(&lines)
        };

        let new_block = new_lines.join("\n");
        let new_text = sel.replace_range(ls, le, &new_block);

        let new_start = ls as u32;
        let new_end = (ls + new_block.len()) as u32;

        (new_text, new_start, new_end)
    }

    fn all_lines_are_wrapped(&self, lines: &[&str]) -> bool {
        lines.iter().all(|l| {
            let trimmed = l.trim_end();
            trimmed.starts_with(self.prefix) && trimmed.ends_with(self.suffix)
        })
    }

    fn add_inline(&self, lines: &[&str]) -> Vec<String> {
        lines
            .iter()
            .map(|l| {
                let trimmed_end = l.trim_end();
                let trailing_spaces = &l[trimmed_end.len()..];
                format!(
                    "{}{}{}{}{}",
                    "",
                    self.prefix,
                    trimmed_end,
                    self.suffix,
                    trailing_spaces
                )
            })
            .collect()
    }

    fn remove_inline(&self, lines: &[&str]) -> Vec<String> {
        lines.iter().map(|l| {
            let trimmed_end = l.trim_end();
            let trailing_spaces = &l[trimmed_end.len()..];

            if trimmed_end.starts_with(self.prefix) && trimmed_end.ends_with(self.suffix) {
                let inner = &trimmed_end[self.prefix.len().. trimmed_end.len() - self.suffix.len()];
                format!("{}{}", inner, trailing_spaces)
            } else {
                (*l).to_string()
            }
        }).collect()
    }
}

impl<'a> SelectionFormatter for Inline<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_inline_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_inline_format_simple() {

        let selection = Selection::new_with_caret_position(
            String::new(),
            0,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "****"
        );
        let start_index_expectation = 2;
        let end_index_expectation = 2;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_different_prefix_and_suffix_simple() {

        let selection = Selection::new_with_caret_position(
            String::new(),
            0,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "![", "](url)").format();

        let text_expectation = String::from(
            "![](url)"
        );
        let start_index_expectation = 2;
        let end_index_expectation = 2;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_caret_in_word() {

        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "I'm a **selected** text \nI'm not :("
        );
        let start_index_expectation = 12;
        let end_index_expectation = 12;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_different_prefix_and_suffix_with_caret_in_word() {

        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "![", "](url)").format();

        let text_expectation = String::from(
            "I'm a ![selected](url) text \nI'm not :("
        );
        let start_index_expectation = 12;
        let end_index_expectation = 12;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_caret_in_surrounded_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I am not selected. \nI'm a selected text \nI am also not selected."),
            20,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "I am not selected. \n**I'm** a selected text \nI am also not selected."
        );
        let start_index_expectation = 22;
        let end_index_expectation = 22;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_one_selected_word() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nI'm not :("),
            Some(String::from("selected ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "I'm a **selected** text \nI'm not :("
        );
        let start_index_expectation = 8;
        let end_index_expectation = 16;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_one_selected_line() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "**I'm a selected text**"
        );
        let start_index_expectation = 2;
        let end_index_expectation = 21;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nMe too!"),
            Some(String::from("I'm a selected text \nMe too!")),  
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "**I'm a selected text** \n**Me too!**"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 36;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_inline_format_with_surrounded_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I am not selected. \nI'm a selected text \nMe too! \nI am also not selected."),
            Some(String::from("I'm a selected text \nMe too! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "I am not selected. \n**I'm a selected text** \n**Me too!** \nI am also not selected."
        );
        let start_index_expectation = 20;
        let end_index_expectation = 57;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_inline_format_simple() {

        let selection = Selection::new_with_caret_position(
            String::from("**Bold**"),
            4
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "Bold"
        );
        let start_index_expectation = 2;
        let end_index_expectation = 2;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_inline_format_with_caret_in_formatted_line() {

        let selection = Selection::new_with_caret_position(
            String::from("**Bold Text**"),
            4
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "****Bold** Text**"
        );
        let start_index_expectation = 6;
        let end_index_expectation = 6;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_inline_format_with_caret_in_formatted_line_2() {

        let selection = Selection::new_with_caret_position(
            String::from("**Bold Text Test**"),
            9
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "**Bold **Text** Test**"
        );
        let start_index_expectation = 11;
        let end_index_expectation = 11;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_inline_format_with_selected_formatted_line() {

        let selection = Selection::new_with_text(
            String::from("**Bold Text**"),
            Some(String::from("Bold Text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "Bold Text"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_inline_format_with_selected_formatted_line_and_prefix_suffix() {

        let selection = Selection::new_with_text(
            String::from("**Bold Text**"),
            Some(String::from("**Bold Text**")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Inline::new(&selection, "**", "**").format();

        let text_expectation = String::from(
            "Bold Text"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
