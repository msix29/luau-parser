use luau_lexer::token::{Keyword, TokenType};
use luau_parser::{parser::Parser, types::{AstStatus, Statement}};

#[test]
fn only_name() {
    let mut parser = Parser::new("local foo");
    let cst = parser.parse(file!());

    assert!(!cst.block.is_empty());
    assert!(cst.errors.is_empty());
    assert_eq!(cst.status, AstStatus::Complete);
    assert!(cst.block.statements[0].1.is_none());
    assert_matches!(&*cst.block.statements[0].0, Statement::LocalAssignment(_));

    get_from_enum!(Statement::LocalAssignment(local_assignment) = &*cst.block.statements[0].0);

    assert_matches!(local_assignment.local_token.token_type, TokenType::Keyword(Keyword::Local));
    assert!(local_assignment.name_list.len() == 1);

    let name = &local_assignment.name_list[0];
    assert_eq!(name.name.token_type, TokenType::Identifier("foo".into()));
    assert!(name.colon.is_none());
    assert!(name.r#type.is_none());

    assert!(local_assignment.equal_token.is_none());
    assert!(local_assignment.expressions.is_empty());
}
