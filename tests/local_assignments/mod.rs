mod table;

use std::sync::Arc;

#[cfg(feature = "raw-values")]
use crate::prelude::HasRawValue;
use luau_parser::{
    get_item_from_tuple_enum,
    prelude::LuauParser,
    types::{
        Expression, FunctionArguments, FunctionCall, FunctionCallInvoked, List, PrefixExp, Print,
        Range, SingleToken, Statement, Var,
    },
};

#[test]
/// Tests simple local assignments.
fn local_assignment_1() {
    let mut parser = LuauParser::new();

    let code = r#"local    foo = "Hello, World!""#;
    let ast = parser.parse(code, "1");
    assert_eq!(ast.statements.len(), 1);
    assert!(matches!(ast.statements[0].0, Statement::LocalAssignment(_)));

    let assignment = get_item_from_tuple_enum!(&ast.statements[0].0, Statement::LocalAssignment);
    assert_eq!(assignment.name_list.items.len(), 1);
    assert_eq!(assignment.name_list.items[0].name.word, "foo");
    assert_eq!(assignment.expressions.items.len(), 1);
    #[cfg(feature = "raw-values")]
    assert_eq!(
        assignment.expressions.items[0].get_raw_value(),
        r#""Hello, World!""#
    );
    assert_eq!(assignment.print(), code);
}

#[test]
/// Tests simple local assignments.
fn local_assignment_2() {
    let mut parser = LuauParser::new();

    let code = r#"local a,    b,    c = 1, foo(   )"#;
    let ast = parser.parse(code, "");
    assert_eq!(ast.statements.len(), 1);
    assert!(matches!(ast.statements[0].0, Statement::LocalAssignment(_)));

    let assignment = get_item_from_tuple_enum!(&ast.statements[0].0, Statement::LocalAssignment);
    assert_eq!(assignment.name_list.items.len(), 3);
    #[cfg(feature = "raw-values")]
    assert_eq!(assignment.name_list.get_raw_value(), "a, b, c");
    assert_eq!(assignment.expressions.items.len(), 2);
    #[cfg(feature = "raw-values")]
    assert_eq!(assignment.expressions.items[0].get_raw_value(), "1");
    assert_eq!(
        *assignment.expressions.items[1],
        Expression::FunctionCall(FunctionCall {
            invoked: FunctionCallInvoked::Function(Arc::new(PrefixExp::Var(Var::Name(
                SingleToken::new("foo")
                    .with_spaces(" ", "")
                    .set_range(Range::new2(0, 25, 0, 28)),
            )))),
            arguments: FunctionArguments::List {
                open_parenthesis: SingleToken::new("(")
                    .with_spaces("", "   ")
                    .set_range(Range::new2(0, 28, 0, 29)),
                arguments: List::default(),
                close_parenthesis: SingleToken::new(")")
                    .with_spaces("   ", "")
                    .set_range(Range::new2(0, 32, 0, 33)),
            },
        })
    );
    assert_eq!(assignment.print(), code);
}
