//! Utility functions.

#[cfg(feature = "lsp-ready")]
mod find_type;
#[cfg(feature = "lsp-ready")]
mod find_variable;

#[cfg(feature = "lsp-ready")]
pub use find_type::*;
#[cfg(feature = "lsp-ready")]
pub use find_variable::*;

use std::str::from_utf8;
use tree_sitter::Node;

use crate::prelude::{Position, Range};

/// Gets the text from a specific byte range in a `&[u8]`, which represents bytes of valid
/// text. This function does check for the passed bytes to ensure they're in the correct
/// range.
pub(crate) fn get_text_from_bytes(bytes: &[u8], start: usize, end: usize) -> String {
    if start > end || bytes.len() < end {
        return "*error*".to_string();
    }

    from_utf8(&bytes[start..end]).unwrap().to_string()
}

/// Get the closest parent that has a sibling, the chosen sibling is decided by the
/// `get_sibling` function. If it returns `Some(Node)`, that value will be returned,
/// else, the function will continue going up the tree till `get_sibling` returns
/// `Some(Node)` or we reach the top of the tree.
fn get_parent_sibling(node: Node, get_sibling: fn(node: Node) -> Option<Node>) -> Option<Node> {
    let mut parent = node.parent();
    while let Some(parent_node) = parent {
        if let Some(sibling) = get_sibling(parent_node) {
            return Some(sibling);
        }

        parent = parent_node.parent();
    }

    None
}

/// Gets spaces before and after a **token**. This function assumes this token has a parent
/// as is only called for individual tokens (ex. `local` in `local foo`).
pub(crate) fn get_spaces(node: Node, code_bytes: &[u8]) -> (String, String) {
    let before = if let Some(before) = node.prev_sibling() {
        // Leading spaces
        get_text_from_bytes(code_bytes, before.end_byte(), node.start_byte())
    } else if let Some(sibling) = get_parent_sibling(node, |node| node.prev_sibling()) {
        // Leading spaces for parent
        get_text_from_bytes(code_bytes, sibling.end_byte(), node.start_byte())
    } else {
        // Leading spaces from the start of the file
        get_text_from_bytes(code_bytes, 0, node.start_byte())
    };

    let after = if let Some(next) = node.next_sibling() {
        // Trailing spaces
        get_text_from_bytes(code_bytes, node.end_byte(), next.start_byte())
    } else if let Some(sibling) = get_parent_sibling(node, |node| node.next_sibling()) {
        // Trailing spaces for parent
        get_text_from_bytes(code_bytes, node.end_byte(), sibling.start_byte())
    } else {
        // Trailing spaces till the end of the file
        get_text_from_bytes(code_bytes, node.end_byte(), code_bytes.len())
    };

    (before.to_string(), after.to_string())
}

/// Get the range of a specific tree-sitter node.
pub fn get_range(node: Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();

    Range {
        start: Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}

/// Get the range of a specific tree-sitter node.
pub(crate) fn get_range_from_boundaries(a: Range, b: Range) -> Range {
    let start = a.start;
    let end = b.end;

    Range {
        start: Position {
            line: start.line,
            character: start.character,
        },
        end: Position {
            line: end.line,
            character: end.character,
        },
    }
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
