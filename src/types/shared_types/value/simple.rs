/// A simple value for the _[value](crate::prelude::Value)_ enum. Simple means it's not a
/// function nor a table.
#[derive(Clone, Debug, Default)]
pub struct SimpleValue {
    /// String representation of the value.
    pub value: String,
}
