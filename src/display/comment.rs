use crate::prelude::{Comment, Print};

impl Print for Comment {
    fn print(&self) -> String {
        self.0.print()
    }
}
