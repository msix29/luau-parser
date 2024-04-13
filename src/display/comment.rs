//! Implements display traits for comments

use crate::prelude::{Comment, Print};

impl Print for Comment {
    fn print(&self) -> String {
        self.0.print()
    }
}
