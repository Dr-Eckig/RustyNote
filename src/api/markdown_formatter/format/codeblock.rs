use super::SelectionFormatter;
use crate::api::markdown_formatter::textarea::Selection;

/// Formatter that toggles fenced code blocks for the selected content.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{CodeBlock, SelectionFormatter};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "let x = 1;".into(),
///     selected_text: Some("let x = 1;".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 10,
/// };
///
/// let (text, _, _) = CodeBlock::new(&selection).format();
/// assert!(text.contains("```"));
/// ```
pub struct CodeBlock<'a> {
    selection: &'a Selection,
}

impl<'a> CodeBlock<'a> {
    /// Creates a code-block formatter for the given selection.
    pub fn new(selection: &'a Selection) -> CodeBlock<'a> {
        CodeBlock { selection }
    }

    fn apply_codeblock_formatting(&self) -> (String, u32, u32) {
        if let Some((open, close_start)) = self.find_surrounding_block() {
            self.unwrap_block(open, close_start)
        } else {
            self.wrap_block()
        }
    }

    fn wrap_block(&self) -> (String, u32, u32) {
        if self.selection.textarea_value.is_empty() {
            return ("```\n\n```\n".to_string(), 4, 4);
        }

        if self.selection.is_empty() {
            let current_line = self.selection.current_line();
            if current_line.trim().is_empty()
                && self.selection.start_index == self.selection.textarea_value.len()
            {
                let mut result = self.selection.textarea_value.clone();
                if !result.ends_with('\n') {
                    result.push('\n');
                }
                let caret_pos = result.len() as u32;
                result.push_str("```\n\n```\n");
                return (result, caret_pos + 4, caret_pos + 4);
            }
            return self.wrap_line_at_caret();
        }

        if self.selection.is_multiline() {
            return self.wrap_multiline_selection();
        }

        if self.selection_is_whole_line() {
            return self.wrap_single_line_selection();
        }

        self.wrap_inline_selection()
    }

    fn wrap_line_at_caret(&self) -> (String, u32, u32) {
        let text = &self.selection.textarea_value;
        let (line_start, line_end) = self.selection.line_bounds();

        let mut after_index = line_end;
        if after_index < text.len() && text.as_bytes()[after_index] == b'\n' {
            after_index += 1;
        }

        let before = &text[..line_start];
        let line = &text[line_start..after_index];
        let after = &text[after_index..];

        if line.trim() == "```" {
            return self.insert_empty_block_at_cursor();
        }

        let mut result = String::new();
        result.push_str(before);

        if !before.is_empty() && !before.ends_with("\n\n") {
            result.push('\n');
        }

        let block_start = result.len();
        result.push_str("```");
        result.push('\n');

        let trimmed_line_is_empty = line.trim().is_empty();

        let caret = if before.is_empty() {
            if trimmed_line_is_empty {
                (block_start + 4) as u32
            } else {
                (block_start + 5) as u32
            }
        } else if trimmed_line_is_empty {
            (block_start + 4) as u32
        } else {
            (block_start + 3) as u32
        };

        result.push_str(line);
        if !line.ends_with('\n') {
            result.push('\n');
        }
        result.push_str("```");
        result.push('\n');

        Self::append_after_block(&mut result, after);

        (result, caret, caret)
    }

    fn wrap_inline_selection(&self) -> (String, u32, u32) {
        let before = self.selection.before();
        let after = self.selection.after();
        let content = self.selection.inner_text().trim();
        let content_len = content.len();

        let mut result = String::new();
        result.push_str(before);
        if !before.ends_with('\n') {
            result.push('\n');
        }

        result.push_str("```");
        result.push('\n');
        let caret_start = (result.len() - 1) as u32;

        result.push_str(content);
        result.push('\n');
        let caret_end = (caret_start as usize + content_len) as u32;

        result.push_str("```");
        result.push('\n');
        result.push_str(after);

        (result, caret_start, caret_end)
    }

    fn wrap_single_line_selection(&self) -> (String, u32, u32) {
        let before = self.selection.before();
        let after = self.selection.after();
        let content = self.selection.inner_text();

        let mut result = String::new();
        result.push_str(before);
        if !before.is_empty() && !before.ends_with("\n\n") {
            if !before.ends_with('\n') {
                result.push('\n');
            }
            result.push('\n');
        }

        let block_start = result.len();
        result.push_str("```");
        result.push('\n');
        result.push_str(content);
        result.push('\n');
        result.push_str("```");
        result.push('\n');

        Self::append_after_block(&mut result, after);

        let caret_start = (block_start + 4) as u32;
        let caret_end = caret_start + content.len() as u32;

        if before.is_empty() && after.is_empty() {
            let caret = (block_start + 2 + content.len()) as u32;
            return (result, caret, caret);
        }

        (result, caret_start, caret_end)
    }

    fn wrap_multiline_selection(&self) -> (String, u32, u32) {
        let before = self.selection.before();
        let after = self.selection.after();
        let selected_text = self.selection.inner_text();
        let original_len = selected_text.len();
        let mut content = selected_text.to_string();
        if !content.ends_with('\n') {
            content.push('\n');
        }

        let mut result = String::new();
        result.push_str(before);
        if !before.is_empty() && !before.ends_with("\n\n") {
            if !before.ends_with('\n') {
                result.push('\n');
            }
            result.push('\n');
        }

        let block_start = result.len();
        result.push_str("```");
        result.push('\n');

        let caret_start = (block_start + 4) as u32;
        result.push_str(&content);
        let caret_end = caret_start + original_len as u32;

        result.push_str("```");
        result.push('\n');

        Self::append_after_block(&mut result, after);

        (result, caret_start, caret_end)
    }

    fn selection_is_whole_line(&self) -> bool {
        if self.selection.is_empty() || self.selection.is_multiline() {
            return false;
        }

        let (line_start, line_end) = self.selection.line_bounds();
        self.selection.start_index == line_start && self.selection.end_index == line_end
    }

    fn unwrap_block(&self, open: usize, close_start: usize) -> (String, u32, u32) {
        let text = &self.selection.textarea_value;
        let close_end = close_start + 3;
        let raw_content = &text[(open + 3)..close_start];
        let leading_newlines = raw_content.chars().take_while(|&c| c == '\n').count();
        let trimmed_start = open + 3 + leading_newlines;
        let inner = raw_content.trim_matches('\n');

        let before_segment = &text[..open];
        let after_segment = &text[close_end..];

        let before_base = before_segment.trim_end_matches('\n');
        let after_base = after_segment.trim_start_matches('\n');

        let removed_before = before_segment.len() - before_base.len();
        let removed_after = after_segment.len() - after_base.len();

        let mut result = String::new();
        result.push_str(before_base);

        if !before_base.is_empty() && !inner.is_empty() {
            let connector = Self::connector_before(before_base, removed_before);
            result.push_str(&connector);
        }

        let insertion_index = result.len();
        result.push_str(inner);

        if !after_base.is_empty() && !inner.is_empty() {
            let connector = Self::connector_after(inner, removed_after);
            result.push_str(&connector);
        }

        result.push_str(after_base);

        let offset = self
            .selection
            .start_index
            .saturating_sub(trimmed_start)
            .min(inner.len());
        let caret_position = (insertion_index + offset) as u32;

        (result, caret_position, caret_position)
    }

    fn connector_before(before_base: &str, removed_newlines: usize) -> String {
        if removed_newlines >= 2 {
            "\n".to_string()
        } else if before_base.ends_with(' ') {
            String::new()
        } else if removed_newlines == 1 {
            " ".to_string()
        } else {
            " ".to_string()
        }
    }

    fn connector_after(inner: &str, removed_newlines: usize) -> String {
        if removed_newlines >= 2 {
            " \n".to_string()
        } else if removed_newlines == 1 {
            " ".to_string()
        } else if inner.ends_with(' ') {
            String::new()
        } else {
            " ".to_string()
        }
    }

    fn find_surrounding_block(&self) -> Option<(usize, usize)> {
        let text = &self.selection.textarea_value;
        let start = self.selection.start_index;
        let end = self.selection.end_index;

        let close_start = if let Some(offset) = text[end..].find("```") {
            end + offset
        } else {
            let search_end = (start + 1).min(text.len());
            text[..search_end].rfind("```")?
        };

        let open = text[..close_start].rfind("```")?;

        if open == close_start {
            return None;
        }

        if open > start || close_start < end {
            return None;
        }

        Some((open, close_start))
    }

    fn insert_empty_block_at_cursor(&self) -> (String, u32, u32) {
        let text = &self.selection.textarea_value;
        let caret = self.selection.start_index;
        let before = &text[..caret];
        let after = &text[caret..];

        let mut result = String::new();
        result.push_str(before);

        if !before.is_empty() && !before.ends_with('\n') {
            result.push('\n');
        }

        let block_start = result.len();
        result.push_str("```");
        result.push('\n');
        result.push('\n');
        result.push_str("```");
        result.push('\n');

        Self::append_after_block(&mut result, after);

        let caret_pos = block_start + 4;

        (result, caret_pos as u32, caret_pos as u32)
    }

    fn append_after_block(result: &mut String, after: &str) {
        if after.is_empty() {
            return;
        }

        if after.starts_with('\n') {
            let mut slice = after;
            while slice.starts_with("\n\n") {
                slice = &slice[1..];
            }
            result.push_str(slice);
        } else if after.starts_with("```") {
            result.push_str(after);
            if after.trim_end().ends_with("```") && !result.ends_with('\n') {
                result.push('\n');
            }
        } else {
            result.push('\n');
            result.push_str(after);
        }
    }
}

impl<'a> SelectionFormatter for CodeBlock<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_codeblock_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_codeblock_simple() {
        let selection = Selection::new_with_caret_position(String::new(), 0);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\n\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 4;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_before_newline() {
        let selection = Selection::new_with_caret_position(String::from("\n"), 0);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\n\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 4;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_simple_with_text_in_textarea() {
        let selection = Selection::new_with_caret_position(
            String::from("Some text in the textarea. \n"),
            28, // Some text in the textarea. \n|
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("Some text in the textarea. \n```\n\n```\n");
        let start_index_expectation = 32;
        let end_index_expectation = 32;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_caret_in_line() {
        let selection = Selection::new_with_caret_position(
            String::from("I'm a selected text \nI'm not :("),
            10,
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nI'm a selected text \n```\n\nI'm not :(");
        let start_index_expectation = 5;
        let end_index_expectation = 5;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_caret_in_surrounded_line() {
        let selection = Selection::new_with_caret_position(
            String::from("I am not selected. \nI'm a selected text \nI am also not selected."),
            21,
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \n\n```\nI'm a selected text \n```\n\nI am also not selected.",
        );
        let start_index_expectation = 24;
        let end_index_expectation = 24;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_one_selected_word() {
        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nI'm not :("),
            Some(String::from("selected ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("I'm a \n```\nselected\n```\ntext \nI'm not :(");
        let start_index_expectation = 10;
        let end_index_expectation = 18;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_one_selected_line() {
        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nI'm a selected text\n```\n");
        let start_index_expectation = 21;
        let end_index_expectation = 21;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_two_selected_lines() {
        let selection = Selection::new_with_text(
            String::from("I'm a selected text \nMe too!"),
            Some(String::from("I'm a selected text \nMe too!")),
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nI'm a selected text \nMe too!\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 32;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_with_surrounded_two_selected_lines() {
        let selection = Selection::new_with_text(
            String::from(
                "I am not selected. \nI'm a selected text \nMe too! \nI am also not selected.",
            ),
            Some(String::from("I'm a selected text \nMe too! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from(
            "I am not selected. \n\n```\nI'm a selected text \nMe too! \n```\n\nI am also not selected.",
        );
        let start_index_expectation = 25;
        let end_index_expectation = 54;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_before_another_codeblock() {
        let selection =
            Selection::new_with_text(String::from("A\n\n```\nB\n```\n"), Some(String::from("A")));

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nA\n```\n\n```\nB\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 5;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_after_another_codeblock() {
        let selection =
            Selection::new_with_text(String::from("```\nA\n```\n\nB"), Some(String::from("B")));

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nA\n```\n\n```\nB\n```\n");
        let start_index_expectation = 15;
        let end_index_expectation = 16;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_directly_before_another_codeblock() {
        let selection = Selection::new_with_caret_position(String::from("```\nA\n```"), 0);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\n\n```\n```\nA\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 4;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_codeblock_directly_after_another_codeblock() {
        let selection = Selection::new_with_caret_position(String::from("```\nA\n```"), 9);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nA\n```\n```\n\n```\n");
        let start_index_expectation = 14;
        let end_index_expectation = 14;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn text_insert_code_block_after_codeblock() {
        let selection = Selection::new_with_text(
            String::from("```\nLet me be\n```\n\nMake me a Codeblock!"),
            Some(String::from("Make me a Codeblock!")),
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation =
            String::from("```\nLet me be\n```\n\n```\nMake me a Codeblock!\n```\n");
        let start_index_expectation = 23;
        let end_index_expectation = 43;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn text_insert_code_block_before_codeblock() {
        let selection =
            Selection::new_with_text(String::from("A\n\n```\nB\n```\n"), Some(String::from("A")));

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("```\nA\n```\n\n```\nB\n```\n");
        let start_index_expectation = 4;
        let end_index_expectation = 5;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_block_prefix_simple() {
        let selection = Selection::new_with_caret_position(String::from("```\nCodeBlock\n```"), 8);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("CodeBlock");
        let start_index_expectation = 4;
        let end_index_expectation = 4;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_block_prefix_with_sourrounded_block() {
        let selection =
            Selection::new_with_caret_position(String::from("Hi! \n```\nCodeBlock\n```\nBye!"), 13);

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("Hi! CodeBlock Bye!");
        let start_index_expectation = 8;
        let end_index_expectation = 8;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_remove_block_prefix_with_sourrounded_block_2() {
        let selection = Selection::new_with_caret_position(
            String::from("Hi! \n\n```\nCodeBlock\n```\n\nBye!"),
            14,
        );

        let (formatted_text, caret_start_index, caret_end_index) =
            CodeBlock::new(&selection).format();

        let text_expectation = String::from("Hi! \nCodeBlock \nBye!");
        let start_index_expectation = 9;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
