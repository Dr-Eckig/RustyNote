use crate::api::markdown_formatter::textarea::Selection;
use super::SelectionFormatter;

/// Formatter that toggles a prefix at the beginning of each selected line.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{BlockPrefix, SelectionFormatter};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "item".into(),
///     selected_text: Some("item".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 4,
/// };
///
/// let (text, start, end) = BlockPrefix::new(&selection, "- ").format();
/// assert_eq!(text, "- item");
/// assert_eq!((start, end), (2, 6));
/// ```
pub struct BlockPrefix<'a> {
    prefix: &'static str,
    selection: &'a Selection,
}

impl<'a> BlockPrefix<'a> {
    /// Creates a formatter that prepends the provided prefix to each selected line.
    pub fn new(selection: &'a Selection, prefix: &'static str) -> Self {
        Self { prefix, selection }
    }

    fn apply_line_prefix_formatting(&self) -> (String, u32, u32) {
        let sel = &self.selection;
        let (block_start, block_end) = sel.line_bounds();
        let lines: Vec<String> = sel.selected_lines().iter().map(|s| s.to_string()).collect();

        let already_prefixed = self.all_lines_have_prefix(&lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
        let new_lines: Vec<String> = if already_prefixed {
            self.remove_prefix(&lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        } else {
            self.add_prefix(&lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        };

        let new_block = new_lines.join("\n");
        let new_text = sel.replace_range(block_start, block_end, &new_block);

        let prefix_len = self.prefix.len();
        let line_count = lines.len();
        let mut new_start = sel.start_index;
        let mut new_end = sel.end_index;

        if already_prefixed {
            if sel.is_empty() {
                new_start = new_start.saturating_sub(prefix_len);
                new_end = new_start;
            } else if line_count == 1 {
                new_start = new_start.saturating_sub(prefix_len);
                new_end = new_end.saturating_sub(prefix_len);
            } else {
                new_end = new_end.saturating_sub(prefix_len * line_count);
            }
        } else {
            if sel.is_empty() {
                new_start += prefix_len;
                new_end = new_start;
            } else if line_count == 1 {
                new_start += prefix_len;
                new_end += prefix_len;
            } else {
                new_end += prefix_len * line_count;
            }
        }

        (new_text, new_start as u32, new_end as u32)
    }

    fn all_lines_have_prefix(&self, lines: &[&str]) -> bool {
        lines.iter().all(|l| l.trim_start().starts_with(self.prefix))
    }

    fn adjust_line_prefix(&self, line: &str, add: bool) -> String {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        if add {
            format!("{}{}{}", " ".repeat(indent), self.prefix, trimmed)
        } else if trimmed.starts_with(self.prefix) {
            format!("{}{}", " ".repeat(indent), &trimmed[self.prefix.len()..])
        } else {
            line.to_string()
        }
    }

    fn add_prefix(&self, lines: &[&str]) -> Vec<String> {
        lines.iter().map(|l| self.adjust_line_prefix(l, true)).collect()
    }

    fn remove_prefix(&self, lines: &[&str]) -> Vec<String> {
        lines.iter().map(|l| self.adjust_line_prefix(l, false)).collect()
    }
}

impl<'a> SelectionFormatter for BlockPrefix<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_line_prefix_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_block_prefix_simple() {

        let selection = Selection::new_with_caret_position(
            String::new(),
            0,
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "- "
        );
        let start_index_expectation = 2;
        let end_index_expectation = 2;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_caret_in_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "- I'm a selected text \nI'm not :("
        );
        let start_index_expectation = 12;
        let end_index_expectation = 12;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_caret_in_surrounded_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I am not selected. \nI'm a selected text \nI am also not selected."),
            20,
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "I am not selected. \n- I'm a selected text \nI am also not selected."
        );
        let start_index_expectation = 22;
        let end_index_expectation = 22;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_one_selected_word() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nI'm not :("),
            Some(String::from("selected ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "- I'm a selected text \nI'm not :("
        );
        let start_index_expectation = 8;
        let end_index_expectation = 17;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_one_selected_line() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "- I'm a selected text"
        );
        let start_index_expectation = 2;
        let end_index_expectation = 21;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nMe too!"),
            Some(String::from("I'm a selected text \nMe too!")),  
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "- I'm a selected text \n- Me too!"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 32;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_block_prefix_with_surrounded_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I am not selected. \nI'm a selected text \nMe too! \nI am also not selected."),
            Some(String::from("I'm a selected text \nMe too! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "I am not selected. \n- I'm a selected text \n- Me too! \nI am also not selected."
        );
        let start_index_expectation = 20;
        let end_index_expectation = 53;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_block_prefix_simple() {

        let selection = Selection::new_with_caret_position(
            String::from("- Unordered List"),
            16
        );

        let (formatted_text, caret_start_index, caret_end_index) = BlockPrefix::new(&selection, "- ").format();

        let text_expectation = String::from(
            "Unordered List"
        );
        let start_index_expectation = 14;
        let end_index_expectation = 14;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
