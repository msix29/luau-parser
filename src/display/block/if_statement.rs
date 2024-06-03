//! Implements helper traits for if statements

use crate::{
    impl_print_struct, optional_print,
    prelude::{ElseIfStatement, ElseStatement, IfStatement},
    print,
};

impl_print_struct!(
    IfStatement,
    { self.if_keyword, print! },
    { self.condition, print! },
    { self.then_keyword, print! },
    { self.body, print! },
    { self.else_if_statements, print! },
    { self.else_statement, optional_print! },
    { self.end_keyword, print! }
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
