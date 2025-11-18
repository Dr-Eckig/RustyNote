use crate::api::markdown_formatter::textarea::Selection;
use super::SelectionFormatter;

/// Formatter that cycles through markdown heading levels for the selected lines.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{Heading, SelectionFormatter};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "Title".into(),
///     selected_text: Some("Title".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 5,
/// };
///
/// let (text, _, _) = Heading::new(&selection).format();
/// assert!(text.starts_with("# "));
/// ```
pub struct Heading<'a> {
    pub selection: &'a Selection,
}

impl<'a> Heading<'a> {
    /// Creates a heading formatter for the given selection.
    pub fn new(selection: &'a Selection) -> Heading<'a> {
        Heading { selection }
    }

    fn apply_heading_formatting(&self) -> (String, u32, u32) {
        let selection = &self.selection;
        let (line_start, line_end) = selection.line_bounds();
        let lines = selection.selected_lines();

        let new_lines: Vec<String> = lines
            .iter()
            .map(|line| self.toggle_heading(line))
            .collect();

        let new_block = new_lines.join("\n");
        let new_text = selection.replace_range(line_start, line_end, &new_block);

        let (new_start, new_end) = self.find_cursor_positions(&new_block);

        (new_text, new_start as u32, new_end as u32)
    }

    fn toggle_heading(&self, line: &str) -> String {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let hash_count = trimmed.chars().take_while(|&c| c == '#').count();

        let content = if hash_count > 0 && trimmed.chars().nth(hash_count) == Some(' ') {
            &trimmed[(hash_count + 1)..]
        } else {
            trimmed
        };

        let new_hash_count = if hash_count >= 5 { 1 } else { hash_count + 1 };
        let new_prefix = "#".repeat(new_hash_count) + " ";

        format!("{}{}{}", " ".repeat(indent), new_prefix, content)
    }

    fn find_cursor_positions(&self, new_block: &str) -> (usize, usize) {
        let selection = &self.selection;
        let (line_start, line_end) = selection.line_bounds();
        let start_line_idx = selection.line_index_of(line_start);

        let selected_lines = selection.selected_lines();

        let first_line = selected_lines.first().unwrap_or(&"");
        let trimmed = first_line.trim_start();
        let old_hash_count = trimmed.chars().take_while(|&c| c == '#').count();
        let new_hash_count = if old_hash_count >= 5 { 1 } else { old_hash_count + 1 };

        let prefix_delta = if new_hash_count == 1 && old_hash_count == 5 {
            -(old_hash_count as isize - 1)
        } else {
            new_hash_count as isize - old_hash_count as isize
        };

        let add_space = if old_hash_count == 0 { 1 } else { 0 };

        let adjust = |pos: usize| -> isize {
            let line_idx = selection.line_index_of(pos) as isize - start_line_idx as isize;
            pos as isize + prefix_delta * (line_idx + 1) + add_space
        };

        let (new_start, new_end) = match &selection.selected_text {
            None => (
                adjust(selection.start_index) as usize,
                adjust(selection.end_index) as usize,
            ),
            Some(_) if selection.start_index == line_start && selection.end_index == line_end => (
                line_start,
                line_start + new_block.len(),
            ),
            Some(_) => (
                adjust(selection.start_index) as usize,
                adjust(selection.end_index) as usize,
            ),
        };

        (new_start, new_end)
    }
}

impl<'a> SelectionFormatter for Heading<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_heading_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_heading_simple() {

        let selection = Selection::new_with_caret_position(
            String::new(),
            0,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# "
        );
        let start_index_expectation = 2;
        let end_index_expectation = 2;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_caret_in_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# I'm a selected text \nI'm not :("
        );
        let start_index_expectation = 12;
        let end_index_expectation = 12;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_caret_in_surrounded_line() {

        let selection = Selection::new_with_caret_position(
            String::from("I am not selected. \nI'm a selected text \nI am also not selected."),
            20,
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \n# I'm a selected text \nI am also not selected."
        );
        let start_index_expectation = 22;
        let end_index_expectation = 22;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_one_selected_word() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nI'm not :("),
            Some(String::from("selected ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# I'm a selected text \nI'm not :("
        );
        let start_index_expectation = 8;
        let end_index_expectation = 17;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_one_selected_line() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# I'm a selected text"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 21;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nMe too!"),
            Some(String::from("I'm a selected text \nMe too!")),  
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# I'm a selected text \n# Me too!"
        );
        let start_index_expectation = 0;
        let end_index_expectation = 32;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_heading_with_surrounded_two_selected_lines() {

        let selection = Selection::new_with_text(
            String::from("I am not selected. \nI'm a selected text \nMe too! \nI am also not selected."),
            Some(String::from("I'm a selected text \nMe too! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \n# I'm a selected text \n# Me too! \nI am also not selected."
        );
        let start_index_expectation = 20;
        let end_index_expectation = 53;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_handle_already_existing_heading_one() {

        let selection = Selection::new_with_caret_position(
            String::from("# Heading"),
            9
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "## Heading"
        );
        let start_index_expectation = 10;
        let end_index_expectation = 10;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_handle_already_existing_heading_two() {

        let selection = Selection::new_with_caret_position(
            String::from("## Heading"),
            10
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "### Heading"
        );
        let start_index_expectation = 11;
        let end_index_expectation = 11;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_handle_already_existing_heading_three() {

        let selection = Selection::new_with_caret_position(
            String::from("### Heading"),
            11
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "#### Heading"
        );
        let start_index_expectation = 12;
        let end_index_expectation = 12;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_handle_already_existing_heading_four() {

        let selection = Selection::new_with_caret_position(
            String::from("#### Heading"),
            12
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "##### Heading"
        );
        let start_index_expectation = 13;
        let end_index_expectation = 13;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_handle_already_existing_heading_five() {

        let selection = Selection::new_with_caret_position(
            String::from("##### Heading"),
            13
        );

        let (formatted_text, caret_start_index, caret_end_index) = Heading::new(&selection).format();

        let text_expectation = String::from(
            "# Heading"
        );
        let start_index_expectation = 9;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
