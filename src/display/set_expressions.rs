//! Set expressions.

use crate::{
    impl_print_struct,
    prelude::{CompoundSetExpression, SetExpression},
    print,
};

impl_print_struct!(
    SetExpression,
    { self.variables, print! },
    { self.equal, print! },
    { self.values, print! }
);
impl_print_struct!(
    CompoundSetExpression,
    { self.variable, print! },
    { self.operation, print! },
    { self.value, print! }
);
