//! Module holding all trait definitions in this parser.

use tree_sitter::{Node, TreeCursor};

use super::Range;

/// A trait for a token that can be represented in a more abstract form for the user to see,
/// without maintaing original styling. This is mainly for LSPs so it's LSP-ready and can
/// be used for things like hover.
#[cfg(feature = "raw-values")]
pub trait HasRawValue {
    /// Get the lossy _raw value_ of this token. For lossless, see [`print`](Print).
    fn get_raw_value(&self) -> String;
}

/// A trait to print the token as-is, while preserving all user spaces and styling.
pub trait Print {
    /// Prints the whole token including all surrounding spaces.
    fn print(&self) -> String;
}

/// A trait to tell Rust that this item is a `LuauStatement`.
pub trait LuauStatement: Sized {
    /// Try creating this [`statement`](LuauStatement) from a [`treesitter node`](Node).
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self>;
}

/// A trait that means this node can be built from a [`tree-sitter Node`](Node).
pub trait FromNode: Sized {
    /// Get the current item from the passed node.
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self>;
}

/// A trait that means this node can be built from a [`tree-sitter Node`](Node).
pub trait FromNodeWithArgs<Args>: Sized {
    /// Get the current item from the passed node.
    fn from_node(node: Node, code_bytes: &[u8], args: Args) -> Option<Self>;
}

/// A trait for getting the range for this specific item.
pub trait HasRange {
    /// Get the range of the node.
    fn get_range(&self) -> Range;
}
