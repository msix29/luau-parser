use luau_lexer::prelude::{Lexer, PartialKeyword, TokenType};
use luau_parser::prelude::{Name, Parse};

#[test]
fn partial_keyword() {
    let mut lexer = Lexer::new("type");
    let name = Name::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    assert!(name.is_some());
    assert_eq!(
        name.unwrap().name.token_type,
        TokenType::PartialKeyword(PartialKeyword::Type)
    );
}

#[test]
fn simple_1() {
    let mut lexer = Lexer::new("name");
    let name = Name::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    assert!(name.is_some());
    assert_eq!(
        name.unwrap().name.token_type,
        TokenType::Identifier("name".into())
    );
}

#[test]
fn simple_2() {
    let mut lexer = Lexer::new("_name");
    let name = Name::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    assert!(name.is_some());
    assert_eq!(
        name.unwrap().name.token_type,
        TokenType::Identifier("_name".into())
    );
}

#[test]
fn faulty() {
    let mut lexer = Lexer::new("123");
    let name = Name::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    assert!(name.is_none());
}
