//! Implements display traits for most of the structs.

mod block;
mod comment;
mod expression;
mod function;
mod list;
mod local_assignment;
mod name;
mod set_expressions;
mod token;
mod type_definition;

use std::sync::Arc;

#[cfg(feature = "raw-values")]
use crate::prelude::HasRawValue;
use crate::prelude::Print;

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

impl<A: Print, B: Print> Print for (A, B) {
    fn print(&self) -> String {
        format!("{}{}", self.0.print(), self.1.print())
    }
}

#[cfg(feature = "raw-values")]
impl<A: HasRawValue, B: HasRawValue> HasRawValue for (A, B) {
    fn get_raw_value(&self) -> String {
        format!("{}{}", self.0.get_raw_value(), self.1.get_raw_value())
    }
}

impl<T: Print> Print for Arc<T> {
    fn print(&self) -> String {
        (**self).print()
    }
}
#[cfg(feature = "raw-values")]
impl<T: HasRawValue> HasRawValue for Arc<T> {
    fn get_raw_value(&self) -> String {
        (**self).get_raw_value()
    }
}
