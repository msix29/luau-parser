//! Implements display traits for most of the structs.

use crate::prelude::Print;

mod block;
mod expression;
mod list;
mod local_assignment;
mod name;
mod token;
mod type_definition;

impl Print for i32 {
    fn print(&self) -> String {
        self.to_string()
    }
}
impl Print for String {
    fn print(&self) -> String {
        self.to_string()
    }
}
impl<T: Print> Print for Vec<T> {
    fn print(&self) -> String {
        self.iter().map(|item| item.print()).collect::<String>()
    }
}
