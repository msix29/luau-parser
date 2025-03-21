use luau_lexer::prelude::{Lexer, TokenType, Literal};
use luau_parser::prelude::{Expression, Parse};

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
            assert!(matches!($var, Expression::$arm1(_)));

            let Expression::$arm1(temp) = $var else {
                unreachable!()
            };

            assert!(matches!(temp.token_type, TokenType::Literal(Literal::$arm2(_))))
        }};
    }

    check!(number, Number, Number);
    check!(string, String, String);
    check!(boolean, Boolean, Boolean);
}
