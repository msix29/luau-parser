//! Implements display traits for do blocks

use crate::{impl_print_struct, print, types::DoBlock};

impl_print_struct!(
    DoBlock,
    { self.do_keyword, print! },
    { self.body, print! },
    { self.end_keyword, print! }
);
