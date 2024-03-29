//! The `call_any` macro.

/// Calls a function from any of the passed `Option<>` items.
#[macro_export]
macro_rules! call_any {
    ($function:ident, $final:expr $(, $option:expr )*) => {{
        $(
            if let Some(item) = &$option {
                return item.$function();
            } else
        )* {
            $final.$function()
        }
    }};
}
