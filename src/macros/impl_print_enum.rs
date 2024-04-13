//! The [`impl_print_enum`] macro.

/// Implements the [`Print`](crate::types::Print) trait for the passed enum.
#[macro_export]
macro_rules! impl_print_enum {
    (
        $struct:ident,
        { $($empty_enum:ident,)* },
        { $($unit_enum:ident,)* },
        {
            $({
                $struct_enum:ident,
                {
                    $($field: ident,)*
                    $( { $optional_field: ident },)*
                }
            },)*
        }
    ) => {
        impl $crate::types::Print for $struct {
            fn print(&self) -> String {
                match self {
                    $( $struct::$empty_enum => String::new(), )*
                    $( $struct::$unit_enum(item) => item.print(), )*
                    $(
                        $struct::$struct_enum { $($field,)* $($optional_field,)* } => {
                            let mut str = String::new();
                            $(str.push_str(&$field.print());)*
                            $(str.push_str(&$crate::optional_print!($optional_field));)*
                            str
                        },
                    )*
                }
            }
        }
    };
}
