//! `get_item_from_enum` macro. Extracts a value from an enum without any checks, use this
//! as a shorthand when you're 100% sure an enum will always match the specific variant.

/// Extracts the value from a tuple enum.
#[macro_export]
macro_rules! get_item_from_tuple_enum {
    ($value:expr, $enum:ident::$variant:ident) => {{
        match $value {
            $enum::$variant(value) => value,
            _ => unreachable!(),
        }
    }};
}
