//! Enums to check if a specific item matches an enum variant.

/// Checks whether or not the passed value matches a specific unit enum variant.
#[macro_export]
macro_rules! unit_enum_matches {
    ($variant:expr, $enum:ident::$check_for:ident) => {{
        match $variant {
            $enum::$check_for => true,
            _ => false
        }
    }};
}

/// Checks whether or not the passed value matches a specific tuple enum variant.
#[macro_export]
macro_rules! tuple_enum_matches {
    ($variant:expr, $enum:ident::$check_for:ident) => {{
        match $variant {
            $enum::$check_for(_) => true,
            _ => false
        }
    }};
}

/// Checks whether or not the passed value matches a specific struct enum variant.
#[macro_export]
macro_rules! struct_enum_matches {
    ($variant:expr, $enum:ident::$check_for:ident) => {{
        match $variant {
            $enum::$check_for { .. } => true,
            _ => false
        }
    }};
}
