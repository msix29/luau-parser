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
