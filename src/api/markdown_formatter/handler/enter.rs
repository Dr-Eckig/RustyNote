use crate::api::markdown_formatter::textarea::{Selection, get_current_selection, set_cursor};

/// Handles Enter key press for smart list continuation and formatting.
///
/// This function reads the current textarea selection, processes it based on context
/// (empty line, list items, etc.), and updates the textarea with the result.
///
/// # Behavior
///
/// - Empty lines: Inserts a simple newline
/// - List items: Continues the list with appropriate numbering/markers
/// - Empty list items: Removes the list marker
/// - Regular text: Inserts a newline
///
/// # Examples
///
/// ```rust,ignore
/// use markdown::api::markdown_formatter::handle_enter_for_lists;
///
/// // Requires a browser environment with a textarea element.
/// handle_enter_for_lists();
/// ```
pub fn handle_enter_for_lists() -> String {
    let selection = get_current_selection();
    let (new_text, new_cursor_position) = handle_enter_with_selection(selection);
    set_cursor(
        new_text.clone(),
        new_cursor_position as u32,
        new_cursor_position as u32,
    );
    new_text
}

fn handle_enter_with_selection(selection: Selection) -> (String, usize) {
    let current_line = selection.current_line();
    let (line_start_byte, _) = selection.line_bounds();
    let cursor_byte_pos = selection.start_index;
    let text = &selection.textarea_value;

    let line_info = LineInfo::from_line(current_line, line_start_byte);

    if line_info.is_empty() {
        return insert_newline_at_cursor(text, cursor_byte_pos);
    }

    match ListType::detect(&line_info.trimmed_content) {
        ListType::Numbered(number) => {
            handle_numbered_list(text, cursor_byte_pos, &line_info, number)
        }
        ListType::Checkbox(marker) => {
            handle_checkbox_list(text, cursor_byte_pos, &line_info, marker)
        }
        ListType::Bullet(marker) => handle_bullet_list(text, cursor_byte_pos, &line_info, marker),
        ListType::None => insert_newline_at_cursor(text, cursor_byte_pos),
    }
}

#[derive(Debug)]
struct LineInfo {
    indent: String,
    trimmed_content: String,
    line_start_byte: usize,
}

impl LineInfo {
    fn from_line(line: &str, line_start_byte: usize) -> Self {
        let indent = extract_leading_whitespace(line);
        let trimmed_content = line.trim_start().to_string();

        LineInfo {
            indent,
            trimmed_content,
            line_start_byte,
        }
    }

    fn is_empty(&self) -> bool {
        self.trimmed_content.is_empty()
    }
}

#[derive(Debug, Clone, Copy)]
enum ListType<'a> {
    Numbered(usize),
    Checkbox(&'a str),
    Bullet(&'a str),
    None,
}

impl<'a> ListType<'a> {
    fn detect(line: &'a str) -> Self {
        if let Some(number) = NumberedListParser::parse(line) {
            return ListType::Numbered(number);
        }

        if let Some(marker) = CheckboxMarkerParser::parse(line) {
            return ListType::Checkbox(marker);
        }

        if let Some(marker) = BulletMarkerParser::parse(line) {
            return ListType::Bullet(marker);
        }

        ListType::None
    }
}

fn handle_numbered_list(
    text: &str,
    cursor_byte_pos: usize,
    line_info: &LineInfo,
    current_number: usize,
) -> (String, usize) {
    let marker_length = NumberedListParser::marker_length(&line_info.trimmed_content);
    let has_content = has_content_after_marker(&line_info.trimmed_content, marker_length);

    if has_content {
        let next_number = current_number + 1;
        let new_marker = format!("{}. ", next_number);
        let insert_text = format!("\n{}{}", line_info.indent, new_marker);
        insert_text_at_position(text, cursor_byte_pos, &insert_text)
    } else {
        let marker_start = line_info.line_start_byte + line_info.indent.len();
        remove_list_marker(text, marker_start, marker_length)
    }
}

fn handle_checkbox_list(
    text: &str,
    cursor_byte_pos: usize,
    line_info: &LineInfo,
    marker: &str,
) -> (String, usize) {
    let marker_length = marker.len();
    let has_content = has_content_after_marker(&line_info.trimmed_content, marker_length);

    if has_content {
        let insert_text = format!("\n{}- [ ] ", line_info.indent);
        insert_text_at_position(text, cursor_byte_pos, &insert_text)
    } else {
        let marker_start = line_info.line_start_byte + line_info.indent.len();
        remove_list_marker(text, marker_start, marker_length)
    }
}

fn handle_bullet_list(
    text: &str,
    cursor_byte_pos: usize,
    line_info: &LineInfo,
    marker: &str,
) -> (String, usize) {
    let marker_length = marker.len();
    let has_content = has_content_after_marker(&line_info.trimmed_content, marker_length);

    if has_content {
        let insert_text = format!("\n{}{}", line_info.indent, marker);
        insert_text_at_position(text, cursor_byte_pos, &insert_text)
    } else {
        let marker_start = line_info.line_start_byte + line_info.indent.len();
        remove_list_marker(text, marker_start, marker_length)
    }
}

struct NumberedListParser;

impl NumberedListParser {
    fn parse(line: &str) -> Option<usize> {
        let digits_end = line.find(|c: char| !c.is_ascii_digit())?;

        if digits_end == 0 {
            return None;
        }

        if line[digits_end..].starts_with(". ") {
            line[..digits_end].parse().ok()
        } else {
            None
        }
    }

    fn marker_length(line: &str) -> usize {
        let digits_end = line
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(line.len());

        if digits_end > 0 && line[digits_end..].starts_with(". ") {
            digits_end + 2
        } else {
            0
        }
    }
}

struct CheckboxMarkerParser;

impl CheckboxMarkerParser {
    fn parse(line: &str) -> Option<&str> {
        const MARKERS: &[&str] = &["- [ ] ", "- [x] ", "- [X] "];

        for marker in MARKERS {
            if line.starts_with(marker) {
                return Some(marker);
            }
        }

        None
    }
}

struct BulletMarkerParser;

impl BulletMarkerParser {
    fn parse(line: &str) -> Option<&str> {
        const MARKERS: &[&str] = &["- ", "* "];

        for marker in MARKERS {
            if line.starts_with(marker) {
                return Some(marker);
            }
        }

        None
    }
}

fn extract_leading_whitespace(line: &str) -> String {
    let trimmed_start = line.trim_start();
    let indent_len = line.len() - trimmed_start.len();
    line[..indent_len].to_string()
}

fn has_content_after_marker(line: &str, marker_length: usize) -> bool {
    if marker_length >= line.len() {
        return false;
    }

    line[marker_length..].trim().len() > 0
}

fn insert_newline_at_cursor(text: &str, position: usize) -> (String, usize) {
    insert_text_at_position(text, position, "\n")
}

fn insert_text_at_position(text: &str, position: usize, insert: &str) -> (String, usize) {
    let clamped_pos = position.min(text.len());
    let safe_pos = find_safe_utf8_boundary(text, clamped_pos);

    let mut new_text = String::with_capacity(text.len() + insert.len());
    new_text.push_str(&text[..safe_pos]);
    new_text.push_str(insert);
    new_text.push_str(&text[safe_pos..]);

    (new_text, safe_pos + insert.len())
}

fn remove_list_marker(text: &str, marker_start: usize, marker_length: usize) -> (String, usize) {
    let safe_start = find_safe_utf8_boundary(text, marker_start);
    let marker_end = (safe_start + marker_length).min(text.len());
    let safe_end = find_safe_utf8_boundary(text, marker_end);

    let mut new_text = String::with_capacity(text.len().saturating_sub(safe_end - safe_start));
    new_text.push_str(&text[..safe_start]);
    new_text.push_str(&text[safe_end..]);

    (new_text, safe_start)
}

fn find_safe_utf8_boundary(text: &str, position: usize) -> usize {
    let clamped = position.min(text.len());

    if clamped == text.len() {
        return clamped;
    }

    if !text.is_char_boundary(clamped) {
        for i in (0..clamped).rev() {
            if text.is_char_boundary(i) {
                return i;
            }
        }
        return 0;
    }

    clamped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(text: &str, cursor: usize) -> (String, usize) {
        let selection = Selection::new(text.to_string(), cursor, cursor);
        handle_enter_with_selection(selection)
    }

    #[test]
    fn test_continues_dash_bullet() {
        let input = "- Item";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "- Item\n- ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_preserves_indentation() {
        let input = "    - Item";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "    - Item\n    - ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_removes_empty_dash_bullet() {
        let input = "- ";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "");
        assert_eq!(position, 0);
    }

    #[test]
    fn test_continues_star_bullet() {
        let input = "* Hello";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "* Hello\n* ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_continues_numbered_list() {
        let input = "3. Test";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "3. Test\n4. ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_removes_empty_numbered_list_marker() {
        let input = "5. ";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "");
        assert_eq!(position, 0);
    }

    #[test]
    fn test_continues_checkbox() {
        let input = "- [ ] Task";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "- [ ] Task\n- [ ] ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_continues_checked_checkbox() {
        let input = "- [x] Task";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "- [x] Task\n- [ ] ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_removes_empty_checkbox() {
        let input = "- [ ] ";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "");
        assert_eq!(position, 0);
    }

    #[test]
    fn test_adds_normal_newline_when_not_in_list() {
        let input = "Hello world";
        let cursor = 5; // after "Hello"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Hello\n world");
        assert_eq!(position, 6);
    }

    #[test]
    fn test_adds_newline_with_multibyte_char_simple() {
        let input = "Ä";
        let cursor = input.len(); // after "Ä"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Ä\n");
        assert_eq!(position, 3);
    }

    #[test]
    fn test_adds_newline_with_multibyte_chars_at_start_and_cursor_at_start() {
        let input = "Äh, Hello world";
        let cursor = 0; // before "Äh"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "\nÄh, Hello world");
        assert_eq!(position, 1);
    }

    #[test]
    fn test_adds_newline_with_multibyte_chars_at_start_and_cursor_at_end() {
        let input = "Äh, Hello world";
        let cursor = input.len(); // after "world"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Äh, Hello world\n");
        assert_eq!(position, cursor + 1);
    }

    #[test]
    fn test_adds_newline_with_multibyte_chars_in_middle_and_cursor_in_middle() {
        let input = "Hello wörld";
        let cursor = 5; // after "Hello"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Hello\n wörld");
        assert_eq!(position, 6);
    }

    #[test]
    fn test_adds_newline_with_multibyte_chars_in_middle_and_cursor_at_end() {
        let input = "Hello wörld";
        let cursor = input.len(); // after "wörld"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Hello wörld\n");
        assert_eq!(position, 13);
    }

    #[test]
    fn test_adds_newline_with_multibyte_chars_at_end() {
        let input = "Hello world! Ä";
        let cursor = input.len(); // after "Ä"
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "Hello world! Ä\n");
        assert_eq!(position, cursor + 1);
    }

    #[test]
    fn test_multibyte_char_in_list() {
        let input = "- Äpfel";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "- Äpfel\n- ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_multibyte_char_after_marker() {
        let input = "1. Öl";
        let cursor = input.len();
        let (new, position) = apply(input, cursor);

        assert_eq!(new, "1. Öl\n2. ");
        assert_eq!(position, new.len());
    }

    #[test]
    fn test_find_safe_utf8_boundary() {
        let text = "Ä";
        // Position 1 wäre mitten im UTF-8-Zeichen (Ä ist 2 Bytes)
        assert_eq!(find_safe_utf8_boundary(text, 0), 0);
        assert_eq!(find_safe_utf8_boundary(text, 1), 0); // Sollte auf 0 zurückgehen
        assert_eq!(find_safe_utf8_boundary(text, 2), 2); // Am Ende ist sicher
    }
}
