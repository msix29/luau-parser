//! Implements display traits for [`numerical for loops`](NumericalFor).

use crate::{impl_print_struct, optional_print, prelude::NumericalFor, print};

impl_print_struct!(
    NumericalFor,
    { self.for_keyword, print! },
    { self.variable, print! },
    { self.equal_keyword, print! },
    { self.start, print! },
    { self.start_comma, print! },
    { self.end, print! },
    { self.end_comma, optional_print! },
    { self.step, optional_print! },
    { self.do_block, print! }
);
