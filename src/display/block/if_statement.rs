//! Implements helper traits for if statements

use crate::{
    impl_print_struct,
    prelude::{ElseIfStatement, ElseStatement, IfStatement},
    print,
};

impl_print_struct!(
    IfStatement,
    { self.if_keyword, print! },
    { self.condition, print! },
    { self.then_keyword, print! },
    { self.body, print! }
);
impl_print_struct!(
    ElseIfStatement,
    { self.elseif_keyword, print! },
    { self.condition, print! },
    { self.then_keyword, print! },
    { self.body, print! }
);
impl_print_struct!(
    ElseStatement,
    { self.else_keyword, print! },
    { self.body, print! }
);
