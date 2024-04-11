use std::sync::Arc;

use luau_parser::{
    get_item_from_tuple_enum,
    prelude::LuauParser,
    types::{
        ExpressionInner, FunctionArguments, FunctionCall, FunctionCallInvoked, HasRawValue, List,
        Location, PrefixExp, SingleToken, Statement, Var,
    },
};

#[test]
/// Tests simple local assignments.
fn local_assignment_1() {
    let mut parser = LuauParser::new();

    let ast = parser.parse(r#"local foo = "Hello, World!""#, "1");
    assert_eq!(ast.tokens.len(), 1);
    assert!(matches!(ast.tokens[0], Statement::LocalAssignment(_)));
    let assignment = get_item_from_tuple_enum!(&ast.tokens[0], Statement::LocalAssignment);
    assert_eq!(assignment.name_list.items.len(), 1);
    assert_eq!(assignment.name_list.items[0].name.word, "foo");
    assert_eq!(assignment.expressions.items.len(), 1);
    assert_eq!(
        assignment.expressions.items[0].get_raw_value(),
        r#""Hello, World!""#
    );
}

#[test]
/// Tests simple local assignments.
fn local_assignment_2() {
    let mut parser = LuauParser::new();

    let ast = parser.parse(r#"local a, b, c = 1, foo()"#, "");
    assert_eq!(ast.tokens.len(), 1);
    assert!(matches!(ast.tokens[0], Statement::LocalAssignment(_)));
    let assignment = get_item_from_tuple_enum!(&ast.tokens[0], Statement::LocalAssignment);
    assert_eq!(assignment.name_list.items.len(), 3);
    assert_eq!(assignment.name_list.get_raw_value(), "a,b,c");
    assert_eq!(assignment.expressions.items.len(), 2);
    assert_eq!(assignment.expressions.items[0].get_raw_value(), "1");
    assert_eq!(
        *assignment.expressions.items[1].inner,
        ExpressionInner::FunctionCall(FunctionCall {
            invoked: FunctionCallInvoked::Function(Arc::new(PrefixExp::Var(Var::Name(
                SingleToken::new("foo")
                    .with_spaces(" ", "")
                    .set_location(Location::new2(0, 19, 0, 22)),
            )))),
            arguments: FunctionArguments::List {
                open_parenthesis: SingleToken::new("(").set_location(Location::new2(0, 22, 0, 23)),
                arguments: List::default(),
                close_parenthesis: SingleToken::new(")").set_location(Location::new2(0, 23, 0, 24)),
            },
        })
    );
}
