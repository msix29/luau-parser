//! The [`impl_print`] macro.

/// Implements the [`Print`](crate::types::Print) trait for the passed struct.
#[macro_export]
macro_rules! impl_print {
    ($struct:ident) => {
        impl $crate::types::Print for $struct {
            fn print(&self) -> String {
                todo!()
            }
        }
    };
}
