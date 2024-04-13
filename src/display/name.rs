//! Implements display traits for names.

use crate::{
    impl_print_struct, optional_print,
    prelude::{HasRawValue, NormalizedName},
    print,
};

impl HasRawValue for NormalizedName {
    fn get_raw_value(&self) -> String {
        if let Some(r#type) = &self.r#type {
            format!("{}: {}", self.name.get_raw_value(), r#type.get_raw_value())
        } else {
            self.name.get_raw_value()
        }
    }
}
impl_print_struct!(
    NormalizedName,
    { self.name, print! },
    { self.colon, optional_print! },
    { self.r#type, optional_print! }
);
