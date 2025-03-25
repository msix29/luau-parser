use luau_lexer::prelude::{Lexer, Literal, PartialKeyword, TokenType};
use luau_parser::prelude::{Expression, Parse, TableAccessKey, TableAccessPrefix, Var};

#[test]
fn literals() {
    let mut lexer = Lexer::new("1 'abc' true");

    let number = Expression::parse(lexer.next_token(), &mut lexer, &mut Vec::new());
    let string = Expression::parse(lexer.next_token(), &mut lexer, &mut Vec::new());
    let boolean = Expression::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    macro_rules! check {
        ($var: ident, $arm1: ident, $arm2: ident) => {{
            assert!($var.is_some());

            let $var = $var.as_ref().unwrap();
            assert_matches!($var, Expression::$arm1(_));

            let Expression::$arm1(temp) = $var else {
                unreachable!()
            };

            assert_matches!(temp.token_type, TokenType::Literal(Literal::$arm2(_)))
        }};
    }

    check!(number, Number, Number);
    check!(string, String, String);
    check!(boolean, Boolean, Boolean);
}

#[test]
fn var() {
    let mut lexer = Lexer::new("type foo.bar qux");

    let name1 = Var::parse(lexer.next_token(), &mut lexer, &mut Vec::new());
    let table_access = Var::parse(lexer.next_token(), &mut lexer, &mut Vec::new());
    let name2 = Var::parse(lexer.next_token(), &mut lexer, &mut Vec::new());

    assert!(name1.is_some());
    assert!(table_access.is_some());
    assert!(name2.is_some());

    let name1 = name1.unwrap();
    let table_access = table_access.unwrap();
    let name2 = name2.unwrap();

    assert_matches!(name1, Var::Name(_));
    assert_matches!(table_access, Var::TableAccess(_));
    assert_matches!(name2, Var::Name(_));

    get_from_enum!(Var::Name(name1) = name1);
    get_from_enum!(Var::TableAccess(table_access) = table_access);
    get_from_enum!(Var::Name(name2) = name2);

    assert_eq!(
        name1.token_type,
        TokenType::PartialKeyword(PartialKeyword::Type)
    );
    assert_eq!(name2.token_type, TokenType::Identifier("qux".into()));

    assert_matches!(table_access.prefix, TableAccessPrefix::Name(_));

    get_from_enum!(TableAccessPrefix::Name(prefix) = table_access.prefix);

    assert!(!table_access.accessed_keys.is_empty());
    assert_matches!(table_access.accessed_keys[0], TableAccessKey::Name { .. });

    get_from_enum!(TableAccessKey::Name { ref name, .. } = table_access.accessed_keys[0]);

    assert_eq!(prefix.token_type, TokenType::Identifier("foo".into()));
    assert_eq!(name.token_type, TokenType::Identifier("bar".into()));
}
