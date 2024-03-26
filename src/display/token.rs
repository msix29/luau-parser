use std::fmt::{Debug, Display};

use crate::{
    impl_print,
    prelude::{HasRawValue, SingleToken},
};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
impl Display for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl Debug for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Word ({})", &self.get_raw_value()))
    }
}

impl_print!(SingleToken, word);
