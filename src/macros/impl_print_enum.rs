//! The [`impl_print_enum`] macro.

/// Implements the [`Print`](crate::types::Print) trait for the passed enum.
#[macro_export]
macro_rules! impl_print_enum {
    ($struct:ident $(, { self.$item:ident, $macro:ident! })*) => {
        impl $crate::types::Print for $struct {
            fn print(&self) -> String {
                let mut str = String::new();

                str
            }
        }
    };
}
