//! Helpful macros for the lsp

mod call_any;
mod enum_matches;
mod get_item_from_enum;
mod impl_print_enum;
mod impl_print_struct;

#[macro_export]
/// Macro used to error when an unhandled variant is met.
macro_rules! unhandled_kind {
    ($variable: ident, $currently_parsing: literal) => {{
        eprintln!(
            "Reached unhandled kind '{}' when parsing `{}`.",
            $variable, $currently_parsing
        );
        None
    }};
}

#[macro_export]
/// Macro used to error when [`get_range`](crate::types::HasRange::get_range) is called.
macro_rules! bad_range {
    ($currently_parsing: literal) => {
        panic!(
            "Attempt to call `get_range` on `{}::ERROR`.",
            $currently_parsing
        )
    };
}
