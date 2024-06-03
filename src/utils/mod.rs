//! Utility functions.

#[cfg(feature = "lsp-ready")]
mod find_type;
#[cfg(feature = "lsp-ready")]
mod find_variable;
mod get_trivia;
mod map_option;

#[cfg(feature = "lsp-ready")]
pub use find_type::*;
#[cfg(feature = "lsp-ready")]
pub use find_variable::*;

use std::str::from_utf8;
use tree_sitter::Node;

use crate::types::Range;
pub(crate) use get_trivia::*;
pub(crate) use map_option::*;

/// Gets the text from a specific byte range in a `&[u8]`, which represents bytes of valid
/// text. This function does check for the passed bytes to ensure they're in the correct
/// range.
#[inline]
pub(crate) fn get_text_from_bytes(bytes: &[u8], start: usize, end: usize) -> String {
    if start > end || bytes.len() < end {
        return "*error*".to_string();
    }

    from_utf8(&bytes[start..end]).unwrap().to_string()
}

/// Get the range of a specific tree-sitter node.
pub fn get_range(node: Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();

    Range::new2(
        start.row as u32,
        start.column as u32,
        end.row as u32,
        end.column as u32,
    )
}

/// Get the range of a specific tree-sitter node.
#[inline]
pub fn get_range_from_boundaries(a: Range, b: Range) -> Range {
    Range::new(a.start, b.end)
}

/// Fix the indentation of a string representing a table.
#[cfg(feature = "raw-values")]
pub(crate) fn fix_table_indentation(raw_value: &str) -> String {
    if raw_value.is_empty() {
        return raw_value.to_string();
    }

    let mut indent = 0;
    raw_value
        .lines()
        .map(|line| {
            let contains_opening = line.contains('{');
            let contains_closing = line.contains('}');
            let ignore = contains_opening && contains_closing;
            if !ignore && contains_closing {
                indent -= 1;
            }

            let indented_line = format!("{}{}", "    ".repeat(indent), line.trim());

            if !ignore && contains_opening {
                indent += 1;
            }

            indented_line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Removes the first and last characters of the string.
pub fn remove_surrounding_pair(string: &str) -> String {
    string[1..(string.len() - 1)].to_string()
}
