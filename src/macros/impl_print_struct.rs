//! The [`impl_print_struct`] macro.

/// Implements the [`Print`](crate::types::Print) trait for the passed struct.
#[macro_export]
macro_rules! impl_print_struct {
    ($struct:ident $(, { self.$item:ident, $macro:ident! })*) => {
        impl $crate::types::Print for $struct {
            fn print(&self) -> String {
                let mut str = String::new();
                $(
                    str.push_str(&$macro!(self.$item));
                )*

                str
            }
        }
    };
}

/// Calls the `print` function of the passed item. This is only to be used with
/// [`impl_print_struct`].
#[macro_export]
macro_rules! print {
    ($item:expr) => {
        $item.print()
    };
}

/// Calls the `print` function of the passed item if it's `Some`, else evaluates to an
/// empty string. This is only to be used with [`impl_print_struct`].
#[macro_export]
macro_rules! optional_print {
    ($item:expr) => {
        if let Some(item) = &$item {
            item.print()
        } else {
            String::new()
        }
    };
}
