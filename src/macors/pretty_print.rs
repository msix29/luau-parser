#[macro_export]
macro_rules! impl_pretty_print {
    ($name:ident, $($field:ident),+) => {
        impl $crate::prelude::PrettyPrint for $name {
            fn pretty_print(&self) -> String {
                let mut result = String::new();
                result.push_str(&self.spaces_before.to_string());
                $(result.push_str(&self.$field.to_string());)+
                result.push_str(&self.spaces_after.to_string());

                result
            }
            fn pretty_print_leading(&self) -> String {
                let mut result = String::new();
                result.push_str(&self.spaces_before.to_string());
                $(result.push_str(&self.$field.to_string());)+

                result
            }
            fn pretty_print_trailing(&self) -> String {
                let mut result = String::new();
                $(result.push_str(&self.$field.to_string());)+
                result.push_str(&self.spaces_after.to_string());

                result
            }
        }
    };
}
