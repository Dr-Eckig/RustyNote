mod textarea;
mod shortcuts;
mod handler;
pub mod format;

pub use shortcuts::setup_shortcuts;
pub use handler::handle_enter_for_lists;

pub(crate) fn combine_text_slices(texts: Vec<&str>, capacity: usize) -> String {
    let mut result = String::with_capacity(capacity);
    for text in texts {
        result.push_str(text);
    }
    result
}
