//! Implements display traits for [`local assignments`](LocalAssignment)

use crate::{impl_print_struct, optional_print, prelude::LocalAssignment, print};

impl_print_struct!(
    LocalAssignment,
    { self.local_token, print! },
    { self.name_list, print! },
    { self.equal_token, optional_print! },
    { self.expressions, print! }
);
