use crate::api::markdown_formatter::{combine_text_slices, textarea::{Selection, line_end_at}};
use super::SelectionFormatter;

/// Formatter that inserts or removes a horizontal rule beneath the selection.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{HorizontalRule, SelectionFormatter};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "Intro".into(),
///     selected_text: Some("Intro".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 5,
/// };
///
/// let (text, _, _) = HorizontalRule::new(&selection).format();
/// assert!(text.contains("---"));
/// ```
pub struct HorizontalRule<'a> {
    selection: &'a Selection,
}

impl<'a> HorizontalRule<'a> {
    /// Creates a formatter that inserts a horizontal rule after the selection.
    pub fn new(selection: &'a Selection) -> HorizontalRule<'a> {
        HorizontalRule {
            selection,
        }
    }

    fn apply_horizontal_rule_formatting(&self) -> (String, u32, u32) {
        let insert_pos = self.determine_insert_position();
        let text = &self.selection.textarea_value;
        let (before_insert, after_insert) = text.split_at(insert_pos);

        let (prefix_newline, suffix_newline) =
            Self::calculate_newlines(before_insert, after_insert);

        let hr_content = String::from("---");
        let hr_block = format!("{}{}{}", prefix_newline, hr_content, suffix_newline);

        self.insert_hr(hr_block, insert_pos, before_insert, after_insert)
    }

    fn insert_hr(
        &self,
        hr_block: String,
        insert_pos: usize,
        before_insert: &str,
        after_insert: &str,
    ) -> (String, u32, u32) {
        let new_text = combine_text_slices(
            vec![
                before_insert,
                &hr_block,
                after_insert,
            ],
            before_insert.len() + hr_block.len() + after_insert.len(),
        );

        let hr_len = hr_block.len();
        let (caret_start, caret_end) = self.adjust_caret_positions(insert_pos, hr_len);

        (new_text, caret_start as u32, caret_end as u32)
    }

    fn determine_insert_position(&self) -> usize {
        let text = &self.selection.textarea_value;
        let start = self.selection.start_index;
        if self.selection.is_empty() {
            let line_end = line_end_at(text, start);
            if line_end < text.len() {
                line_end + 1
            } else {
                line_end
            }
        } else {
            let end = self.selection.end_index;
            if end == 0 {
                0
            } else {
                let last_index = end - 1;
                let line_end = line_end_at(text, last_index);
                if line_end < text.len() {
                    line_end + 1
                } else {
                    line_end
                }
            }
        }
    }

    fn adjust_caret_positions(&self, insert_pos: usize, hr_len: usize) -> (usize, usize) {
        let mut start = self.selection.start_index;
        let mut end = self.selection.end_index;

        if insert_pos <= start {
            start += hr_len;
            end += hr_len;
        } else if insert_pos < end {
            end += hr_len;
        }

        (start, end)
    }

    fn calculate_newlines(before: &str, after: &str) -> (String, String) {
        let prefix = if before.is_empty() {
            ""
        } else if before.ends_with("\n\n") {
            ""
        } else if before.ends_with('\n') {
            "\n"
        } else {
            "\n\n"
        };

        let suffix = if after.is_empty() {
            "\n\n"
        } else if after.starts_with("\n\n") {
            ""
        } else if after.starts_with('\n') {
            "\n"
        } else {
            "\n\n"
        };

        (prefix.to_string(), suffix.to_string())
    }
}

impl<'a> SelectionFormatter for HorizontalRule<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_horizontal_rule_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_horizontal_rule_simple() {

        let selection = Selection::new_with_caret_position(
            String::new(),
            0,
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "---\n\n"
        );
        let start_index_expectation = 5;
        let end_index_expectation = 5;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_caret_in_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I'm a selected text \n\n---\n\nI'm not :("
        );
        let start_index_expectation = 10;
        let end_index_expectation = 10;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_caret_in_surrounded_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I am not selected. \nI'm a selected text \nI am also not selected."),
            20,
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \nI'm a selected text \n\n---\n\nI am also not selected."
        );
        let start_index_expectation = 20;
        let end_index_expectation = 20;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_one_selected_word() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nI'm not :("),
            Some(String::from("selected ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I'm a selected text \n\n---\n\nI'm not :("
        );
        let start_index_expectation = 6;
        let end_index_expectation = 15;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_one_selected_line() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I'm a selected text\n\n---\n\n"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 19;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nMe too!"),
            Some(String::from("I'm a selected text \nMe too!")),  
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I'm a selected text \nMe too!\n\n---\n\n"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 28;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_horizontal_rule_with_surrounded_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I am not selected. \nI'm a selected text \nMe too! \nI am also not selected."),
            Some(String::from("I'm a selected text \nMe too! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = HorizontalRule::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \nI'm a selected text \nMe too! \n\n---\n\nI am also not selected."
        );
        let start_index_expectation = 20;
        let end_index_expectation = 49;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
