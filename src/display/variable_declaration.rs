//! Implements display traits for _[variable declarations](LocalAssignment)_.

use std::fmt::Display;

use crate::prelude::{HasRawValue, LocalAssignment, Print};

impl Display for LocalAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for LocalAssignment {
    fn get_raw_value(&self) -> String {
        if let Some(equal) = &self.equal_token {
            format!(
                "{} {} {} {}",
                self.local_token,
                self.name_list.join(),
                equal,
                self.expressions.join()
            )
        } else {
            format!("{} {}", self.local_token, self.name_list.join())
        }
    }
}

impl Print for LocalAssignment {
    fn print(&self) -> String {
        if let Some(equal) = &self.equal_token {
            format!(
                "{}{}{}{}",
                self.local_token.print(),
                self.name_list.join(),
                equal,
                self.expressions.join()
            )
        } else {
            format!("{}{}", self.local_token.print(), self.name_list.join())
        }
    }
    fn print_leading(&self) -> String {
        todo!()
    }
    fn print_trailing(&self) -> String {
        todo!()
    }
}
