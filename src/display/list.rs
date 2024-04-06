use crate::prelude::{HasRawValue, List};

impl<T: HasRawValue> HasRawValue for List<T> {
    fn get_raw_value(&self) -> String {
        self.items
            .iter()
            .map(|item| item.get_raw_value())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
