//! All `impl` blocks for [`Name`].

use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};
use smol_str::SmolStr;

use crate::types::{Name, Parse, Pointer, TryParse, TypeValue};

impl Name {
    /// An error name that should be used when a name failed to parse but must exist.
    pub const ERROR: Self = Self {
        name: Token::empty(TokenType::Identifier(SmolStr::new_inline("*error*"))),
        colon: None,
        r#type: None,
    };
}

impl Parse for Name {
    fn parse(name: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            name.token_type,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_)
        ) {
            return None;
        }

        maybe_next_token!(lexer, colon, TokenType::Symbol(Symbol::Colon));

        let r#type = if colon.is_some() {
            Pointer::<TypeValue>::try_parse(lexer, errors)
        } else {
            None
        };

        Some(Self {
            name,
            colon,
            r#type,
        })
    }
}
impl TryParse for Name {}
