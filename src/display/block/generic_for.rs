//! For in struct.

use crate::{impl_print_struct, prelude::GenericFor, print};

impl_print_struct!(
    GenericFor,
    { self.for_keyword, print! },
    { self.names, print! },
    { self.in_keyword, print! },
    { self.expressions, print! },
    { self.do_block, print! }
);
