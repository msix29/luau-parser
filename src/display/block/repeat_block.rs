//! Repeat blocks.

use crate::{impl_print_struct, prelude::RepeatBlock, print};

impl_print_struct!(
    RepeatBlock,
    { self.repeat_keyword, print! },
    { self.body, print! },
    { self.until_keyword, print! },
    { self.condition, print! }
);
