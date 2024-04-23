//! Local and global functions.

use crate::{
    impl_print_enum, impl_print_struct, optional_print,
    prelude::{GlobalFunction, GlobalFunctionName, LocalFunction},
    print,
};

impl_print_struct!(
    LocalFunction,
    { self.local_keyword, print! },
    { self.function_keyword, print! },
    { self.function_name, print! },
    { self.generics, optional_print! },
    { self.opening_parenthesis, print! },
    { self.parameters, print! },
    { self.closing_parenthesis, print! },
    { self.returns, optional_print! },
    { self.body, print! },
    { self.end_keyword, print! }
);

impl_print_enum!(
    GlobalFunctionName,
    {},
    { SimpleName, },
    {
        {
            Table,
            {
                { table, print! },
                { keys, print! },
                { method, optional_print! },
            }
        },
    }
);

impl_print_struct!(
    GlobalFunction,
    { self.function_keyword, print! },
    { self.function_name, print! },
    { self.generics, optional_print! },
    { self.opening_parenthesis, print! },
    { self.parameters, print! },
    { self.closing_parenthesis, print! },
    { self.returns, optional_print! },
    { self.body, print! },
    { self.end_keyword, print! }
);
