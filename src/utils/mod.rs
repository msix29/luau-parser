//! Utility functions.

use std::str::from_utf8;
use tree_sitter::Node;

use crate::prelude::{Location, Position};

/// Gets the text from a specific location in a `&[u8]`, which represents bytes of valid
/// text. This function does check for the passed bytes to ensure they're in the correct
/// range.
pub(crate) fn get_text_from_bytes(bytes: &[u8], start: usize, end: usize) -> String {
    if start > end || bytes.len() <= end {
        return "*error*".to_string();
    }

    from_utf8(&bytes[start..end]).unwrap().to_string()
}

/// Gets spaces before and after a **token**. This function assumes this token has a parent
/// as is only called for individual tokens (ex. `local` in `local foo`).
pub(crate) fn get_spaces(node: Node, code_bytes: &[u8]) -> (String, String) {
    let before = if let Some(before) = node.prev_sibling() {
        // Leading spaces
        get_text_from_bytes(code_bytes, before.end_byte(), node.start_byte())
    } else if let Some(before_parent) = node.parent().unwrap().prev_sibling() {
        // Leading spaces for parent
        get_text_from_bytes(code_bytes, before_parent.end_byte(), node.start_byte())
    } else {
        // Leading spaces from the start of the file
        get_text_from_bytes(code_bytes, 0, node.start_byte())
    };

    let after = if let Some(next) = node.next_sibling() {
        // Trailing spaces
        get_text_from_bytes(code_bytes, node.end_byte(), next.start_byte())
    } else if let Some(next_parent) = node.parent().unwrap().next_sibling() {
        // Trailing spaces for parent
        get_text_from_bytes(code_bytes, node.end_byte(), next_parent.start_byte())
    } else {
        // Trailing spaces till the end of the file
        get_text_from_bytes(code_bytes, node.end_byte(), code_bytes.len())
    };

    (before.to_string(), after.to_string())
}

/// Get the location of a specific tree-sitter node.
pub fn get_location(node: Node) -> Location {
    let start = node.start_position();
    let end = node.end_position();

    Location {
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

/// Get the location of a specific tree-sitter node.
pub(crate) fn get_location_from_boundaries(a: Location, b: Location) -> Location {
    let start = a.start;
    let end = b.end;

    Location {
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

            let indented_line = format!("{}{}", "\t".repeat(indent), line.trim());

            if !ignore && contains_opening {
                indent += 1;
            }

            indented_line
        })
        .collect::<Vec<_>>()
        .join("\n")
}
