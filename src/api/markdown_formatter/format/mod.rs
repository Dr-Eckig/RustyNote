use crate::api::markdown_formatter::textarea::{get_current_selection, set_cursor};

mod inline;
mod block_prefix;
mod heading;
mod codeblock;
mod ordered_list;
mod table;
mod horizontal_rule;

pub use self::{
    block_prefix::BlockPrefix,
    codeblock::CodeBlock,
    heading::Heading,
    horizontal_rule::HorizontalRule,
    inline::Inline,
    ordered_list::OrderedList,
    table::Table,
};

/// Shared interface implemented by every formatter.
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
pub trait SelectionFormatter {
    fn format(&self) -> (String, u32, u32);
}

pub enum TextFormattingType {
    Inline {
        prefix: &'static str,
        suffix: &'static str,
    },
    LinePrefix {
        prefix: &'static str,
    },
    Heading,
    CodeBlock,
    OrderedList,
    Table, 
    HorizontalRule,
}

impl TextFormattingType {

    /// Applies the formatter represented by the enum variant to the current textarea selection.
    ///
    /// ```rust,ignore
    /// use markdown::api::markdown_formatter::format::TextFormattingType;
    ///
    /// // Requires a browser environment with an element that exposes a selection.
    /// let formatted = TextFormattingType::Heading.apply_text_formatting();
    /// assert!(!formatted.is_empty());
    /// ```
    pub fn apply_text_formatting(&self) -> String {

        let selection = get_current_selection();

        let (new_value, new_sel_start, new_sel_end) = match self {
            TextFormattingType::Inline { prefix, suffix } => {
                Inline::new(&selection, prefix, suffix).format()
            }

            TextFormattingType::LinePrefix { prefix } => {
                BlockPrefix::new(&selection, prefix).format()
            }

            TextFormattingType::Heading => {
                Heading::new(&selection).format()
            }

            TextFormattingType::CodeBlock => {
                CodeBlock::new(&selection).format()
            }

            TextFormattingType::HorizontalRule => {
                HorizontalRule::new(&selection).format()
            }

            TextFormattingType::OrderedList => {
                OrderedList::new(&selection).format()
            }

            TextFormattingType::Table => {
                Table::new(&selection).format()
            }
        };

        set_cursor(new_value, new_sel_start, new_sel_end)
    }
}
