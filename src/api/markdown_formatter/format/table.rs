use markdown_table_formatter::format_tables;

use super::SelectionFormatter;
use crate::api::markdown_formatter::textarea::Selection;

/// Formatter that generates a markdown table scaffold from the selected text.
///
/// ```rust
/// use markdown::api::markdown_formatter::format::{SelectionFormatter, Table};
/// use markdown::api::markdown_formatter::textarea::Selection;
///
/// let selection = Selection {
///     textarea_value: "Name\nEmail".into(),
///     selected_text: Some("Name\nEmail".into()),
///     before_selection: String::new(),
///     after_selection: String::new(),
///     start_index: 0,
///     end_index: 9,
/// };
///
/// let (text, _, _) = Table::new(&selection).format();
/// assert!(text.contains("| Name "));
/// ```
pub struct Table<'a> {
    selection: &'a Selection,
}

impl<'a> Table<'a> {
    /// Creates a table formatter for the given selection.
    pub fn new(selection: &'a Selection) -> Self {
        Self { selection }
    }

    fn apply_table_formatting(&self) -> (String, u32, u32) {
        let Selection {
            selected_text,
            before_selection,
            after_selection,
            ..
        } = &self.selection;

        let (headers, before_clean, after_clean, caret_override) =
            if let Some(selected) = selected_text {
                self.handle_multiline_selection(selected, before_selection, after_selection)
            } else {
                self.handle_cursor_position(before_selection, after_selection)
            };

        let table_text = self.build_table_text(&headers);
        let new_text = self.build_new_text(&before_clean, &table_text, &after_clean);
        let (start, end) =
            self.calculate_cursor_positions(&before_clean, &table_text, &headers, caret_override);
        (new_text, start, end)
    }

    fn handle_multiline_selection(
        &self,
        selected_text: &str,
        before_selection: &str,
        after_selection: &str,
    ) -> (Vec<String>, String, String, Option<usize>) {
        let before = before_selection
            .trim_end_matches('\n')
            .trim_end()
            .to_string();
        let after = after_selection
            .trim_start_matches('\n')
            .trim_start()
            .to_string();

        let mut lines: Vec<String> = selected_text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        match lines.len() {
            0 => {
                lines.push("Header1".into());
                lines.push("Header2".into());
            }
            1 => {
                lines.push("Header2".into());
            }
            _ => {
                let next_index = lines.len() + 1;
                lines.push(format!("Header{next_index}"));
            }
        }

        let caret_index = lines.len().saturating_sub(1);

        (lines, before, after, Some(caret_index))
    }

    fn handle_cursor_position(
        &self,
        before_selection: &str,
        after_selection: &str,
    ) -> (Vec<String>, String, String, Option<usize>) {
        let cursor_is_at_newline_boundary = before_selection.ends_with('\n')
            || after_selection.starts_with('\n')
            || (before_selection.is_empty() && after_selection.starts_with('\n'))
            || (after_selection.is_empty() && before_selection.ends_with('\n'));

        if cursor_is_at_newline_boundary {
            let before_trimmed = before_selection.trim_end().to_string();
            let appending_to_table = is_appending_to_table(&before_trimmed);
            return (
                vec!["Header1".into(), "Header2".into()],
                before_trimmed,
                after_selection.trim_start().to_string(),
                if appending_to_table { Some(0) } else { None },
            );
        }

        let before_trim = before_selection.trim_end();
        let after_trim = after_selection.trim_start();

        let word_start = self.find_word_start(before_trim);
        let before_word = &before_trim[word_start..];
        let after_word = self.extract_word_with_punctuation(after_trim);

        let header = if !before_word.is_empty() || !after_word.is_empty() {
            format!("{}{}", before_word, after_word)
        } else {
            "Header1".into()
        };

        let before_clean = before_trim[..word_start].trim_end().to_string();
        let after_clean = if after_word.len() < after_trim.len() {
            after_trim[after_word.len()..].trim_start().to_string()
        } else {
            String::new()
        };

        (
            vec![header, "Header2".into()],
            before_clean,
            after_clean,
            None,
        )
    }

    fn find_word_start(&self, text: &str) -> usize {
        text.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0)
    }

    fn extract_word_with_punctuation(&self, text: &str) -> String {
        text.chars().take_while(|c| !c.is_whitespace()).collect()
    }

    fn build_table_text(&self, headers: &[String]) -> String {
        let header_row = headers
            .iter()
            .map(|h| format!("| {} ", h))
            .collect::<String>()
            + "|";
        let separator_row = headers.iter().map(|_| "|----------").collect::<String>() + "|";
        let cell_row = headers
            .iter()
            .enumerate()
            .map(|(i, _)| format!("| Cell{} ", i + 1))
            .collect::<String>()
            + "|";

        format!("{header_row}\n{separator_row}\n{cell_row}\n\n")
    }

    fn build_new_text(&self, before: &str, table: &str, after: &str) -> String {
        let mut out = String::new();
        #[allow(clippy::unnecessary_to_owned)]
        let table = format_tables(table.to_string());

        if !before.is_empty() {
            out.push_str(before);
            out.push_str("\n\n");
        }

        out.push_str(&table);

        if !after.is_empty() {
            out.push_str(after);
        }

        out
    }

    fn calculate_cursor_positions(
        &self,
        before_clean: &str,
        _table: &str,
        headers: &[String],
        caret_override: Option<usize>,
    ) -> (u32, u32) {
        let table_insertion_offset = if before_clean.is_empty() {
            0
        } else {
            // when there is text before, build_new_text inserts two newlines
            before_clean.len() + 2
        };

        let target_index = determine_target_header_index(headers, caret_override);
        let header_start_offset = header_start_offset(headers, target_index);
        let header_len = headers.get(target_index).map(|h| h.len()).unwrap_or(0);

        let start = table_insertion_offset + header_start_offset;
        let end = start + header_len;

        (start as u32, end as u32)
    }
}

fn determine_target_header_index(headers: &[String], override_index: Option<usize>) -> usize {
    if headers.is_empty() {
        return 0;
    }
    if let Some(index) = override_index {
        return index.min(headers.len() - 1);
    }
    if headers.len() >= 3 {
        return headers.len() - 1;
    }
    if headers
        .first()
        .map(|h| h.starts_with("Header1"))
        .unwrap_or(false)
    {
        0
    } else if headers.len() > 1 {
        1
    } else {
        0
    }
}

fn header_start_offset(headers: &[String], target_index: usize) -> usize {
    let mut offset = 0;
    for (idx, header) in headers.iter().enumerate() {
        offset += 2; // account for "| "
        if idx == target_index {
            return offset;
        }
        offset += header.len() + 1; // skip header text and trailing space
    }
    offset
}

fn is_appending_to_table(text: &str) -> bool {
    let mut lines = text.lines().rev();
    let cell_line = match lines.next() {
        Some(line) => line.trim(),
        None => return false,
    };
    if !is_table_row(cell_line) {
        return false;
    }

    let separator_line = match lines.next() {
        Some(line) => line.trim(),
        None => return false,
    };
    if !is_separator_row(separator_line) {
        return false;
    }

    let header_line = match lines.next() {
        Some(line) => line.trim(),
        None => return false,
    };
    is_table_row(header_line)
}

fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}

fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.contains('-')
}

impl<'a> SelectionFormatter for Table<'a> {
    fn format(&self) -> (String, u32, u32) {
        self.apply_table_formatting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::markdown_formatter::format::SelectionFormatter;

    #[test]
    fn test_insert_table_simple() {
        let selection = Selection::new_with_caret_position(String::new(), 0);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation =
            String::from("| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n");
        let start_index_expectation = 2;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_caret_in_single_word() {
        let selection = Selection::new_with_caret_position(String::from("Test"), 2);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation =
            String::from("| Test | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n");
        let start_index_expectation = 9;
        let end_index_expectation = 16;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_caret_in_text_simple() {
        let selection = Selection::new_with_caret_position(String::from("I'm a Test."), 8);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "I'm a\n\n| Test. | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n",
        );
        let start_index_expectation = 17;
        let end_index_expectation = 24;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_caret_in_text() {
        let selection =
            Selection::new_with_caret_position(String::from("I'm a Test. Have fun!"), 8);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "I'm a\n\n| Test. | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\nHave fun!",
        );
        let start_index_expectation = 17;
        let end_index_expectation = 24;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_caret_in_new_line_at_the_start() {
        let selection = Selection::new_with_caret_position(String::from("\nI'm a Test."), 0);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\nI'm a Test.",
        );
        let start_index_expectation = 2;
        let end_index_expectation = 9;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_caret_in_new_line_at_the_end() {
        let selection = Selection::new_with_caret_position(String::from("I'm a Test.\n"), 12);

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "I'm a Test.\n\n| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n",
        );
        let start_index_expectation = 15;
        let end_index_expectation = 22;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_selected_text_simple() {
        let selection = Selection::new_with_text(
            String::from("I'm a selected text"),
            Some(String::from("I'm a selected text")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "| I'm a selected text | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n",
        );
        let start_index_expectation = 24;
        let end_index_expectation = 31;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_selected_word() {
        let selection = Selection::new_with_text(
            String::from("Hello, I'm a selected text. Nice to meet you!"),
            Some(String::from("I'm a selected text. ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "Hello,\n\n| I'm a selected text. | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\nNice to meet you!",
        );
        let start_index_expectation = 33;
        let end_index_expectation = 40;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_one_selected_line() {
        let selection = Selection::new_with_text(
            String::from("Hello, \nI'm a selected text. \nNice to meet you!"),
            Some(String::from("I'm a selected text. ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "Hello,\n\n| I'm a selected text. | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\nNice to meet you!",
        );
        let start_index_expectation = 33;
        let end_index_expectation = 40;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_two_selected_lines() {
        let selection = Selection::new_with_text(
            String::from("Hello, \nI'm a selected text. \nNice to meet you! \n:)"),
            Some(String::from("I'm a selected text. \nNice to meet you! ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "Hello,\n\n| I'm a selected text. | Nice to meet you! | Header3 |\n|----------|----------|----------|\n| Cell1 | Cell2 | Cell3 |\n\n:)",
        );
        let start_index_expectation = 53;
        let end_index_expectation = 60;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_three_selected_lines() {
        let selection = Selection::new_with_text(
            String::from("Hello, \nI'm a selected text. \nNice to meet you! \nHave Fun! \n:)"),
            Some(String::from(
                "I'm a selected text. \nNice to meet you! \nHave Fun! ",
            )),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "Hello,\n\n| I'm a selected text. | Nice to meet you! | Have Fun! | Header4 |\n|----------|----------|----------|----------|\n| Cell1 | Cell2 | Cell3 | Cell4 |\n\n:)",
        );
        let start_index_expectation = 65;
        let end_index_expectation = 72;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_with_emoji() {
        let selection = Selection::new_with_text(
            String::from("Hello, I'm a selected text. 😎 Nice to meet you!"),
            Some(String::from("I'm a selected text. ")),
        );

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "Hello,\n\n| I'm a selected text. | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n😎 Nice to meet you!",
        );
        let start_index_expectation = 33;
        let end_index_expectation = 40;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }

    #[test]
    fn test_insert_table_after_table() {
        let input =
            String::from("| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n");

        let selection = Selection::new_with_caret_position(input.clone(), input.len());

        let (formatted_text, caret_start_index, caret_end_index) = Table::new(&selection).format();

        let text_expectation = String::from(
            "| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n| Header1 | Header2 |\n|----------|----------|\n| Cell1 | Cell2 |\n\n",
        );
        let start_index_expectation = 67;
        let end_index_expectation = 74;

        assert_eq!(formatted_text, text_expectation);
        assert_eq!(caret_start_index, start_index_expectation);
        assert_eq!(caret_end_index, end_index_expectation);
    }
}
