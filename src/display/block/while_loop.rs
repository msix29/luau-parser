//! While loops.

use crate::{impl_print_struct, prelude::WhileLoop, print};

impl_print_struct!(
    WhileLoop,
    { self.while_keyword, print! },
    { self.condition, print! },
    { self.do_block, print! }
);
