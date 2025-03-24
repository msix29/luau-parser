use luau_lexer::prelude::{
    Lexer, Literal, Operator, ParseError, PartialKeyword, Symbol, Token, TokenType,
};
use std::sync::Arc;

use crate::{
    force_parse_bracketed, handle_error_token,
    types::{
        Bracketed, BracketedList, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, GenericParameters, Parse, ParseWithArgs, Table,
        TypeDefinition, TypeValue,
    },
};

impl TypeValue {
    fn parse_from_name(
        base: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        let state = lexer.save_state();
        let maybe_dot = lexer.next_token();

        let actual_type = if maybe_dot == TokenType::Symbol(Symbol::Dot) {
            next_token_recoverable!(
                lexer,
                name,
                TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                TokenType::Identifier("*error*".to_string()),
                errors,
                "Expected <ident>"
            );

            Some((maybe_dot.clone(), name.clone()))
        } else {
            lexer.set_state(state);
            None
        };

        maybe_next_token!(
            lexer,
            opening_angle_brackets,
            TokenType::Symbol(Symbol::OpeningAngleBrackets)
        );
        let generics = if opening_angle_brackets.is_some() {
            BracketedList::<Arc<TypeValue>>::parse_with(
                lexer.next_token(),
                lexer,
                errors,
                ("Expected <type>", Symbol::ClosingAngleBrackets),
            )
        } else {
            None
        }
        .map(Box::new);

        if let Some((dot, name)) = actual_type {
            Some(Self::Module {
                module: base,
                dot,
                name,
                generics,
            })
        } else {
            maybe_next_token!(lexer, ellipsis, TokenType::Symbol(Symbol::Ellipses));

            if let Some(ellipsis) = ellipsis {
                Some(Self::GenericPack {
                    name: base,
                    ellipsis,
                })
            } else {
                Some(Self::Basic { base, generics })
            }
        }
    }

    fn parse_inner(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Literal(ref literal) => match literal {
                Literal::Number(_) => None,
                Literal::String(_) => Some(Self::String(token)),
                Literal::Boolean(_) => Some(Self::Boolean(token)),
            },
            TokenType::Identifier(_) => Self::parse_from_name(token, lexer, errors),
            TokenType::PartialKeyword(PartialKeyword::TypeOf) => Some(Self::Typeof {
                typeof_token: token,
                inner: force_parse_bracketed!(
                    lexer,
                    errors,
                    "Expected <expr>",
                    (
                        TokenType::Symbol(Symbol::OpeningParenthesis),
                        TokenType::Symbol(Symbol::OpeningParenthesis)
                    ),
                    Symbol::ClosingParenthesis,
                ),
            }),
            TokenType::PartialKeyword(_) => Self::parse_from_name(token, lexer, errors),
            TokenType::Symbol(Symbol::OpeningCurlyBrackets) => {
                Table::parse_with(token, lexer, errors, true).map(Self::Table)
            }
            TokenType::Symbol(Symbol::OpeningParenthesis) => {
                if let Some(bracketed) = BracketedList::<Arc<TypeValue>>::parse_with(
                    token,
                    lexer,
                    errors,
                    ("Expected <type>", Symbol::ClosingParenthesis),
                ) {
                    if bracketed.items.len() == 1 {
                        Some(Self::Wrap(Bracketed {
                            item: (*bracketed.items[0]).clone(), // different order to fix deref issues.
                            opening_bracket: bracketed.opening_bracket,
                            closing_bracket: bracketed.closing_bracket,
                        }))
                    } else {
                        Some(Self::Tuple(bracketed))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Parse for TypeValue {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let left = Self::parse_inner(token, lexer, errors)?;
        let state = lexer.save_state();
        let maybe_operator = lexer.next_token();

        match maybe_operator.token_type {
            TokenType::Operator(Operator::Optional) => Some(Self::Optional {
                base: Arc::new(left),
                question_mark: maybe_operator,
            }),
            TokenType::Operator(Operator::Intersection) => Some(Self::Intersection {
                left: Arc::new(left),
                ampersand: maybe_operator,
                right: Self::parse(lexer.next_token(), lexer, errors)
                    .map(Arc::new)
                    .unwrap_or_default(),
            }),
            TokenType::Operator(Operator::Union) => Some(Self::Union {
                left: Arc::new(left),
                pipe: maybe_operator,
                right: Self::parse(lexer.next_token(), lexer, errors)
                    .map(Arc::new)
                    .unwrap_or_default(),
            }),
            _ => {
                lexer.set_state(state);
                Some(left)
            }
        }
    }
}

impl Parse for TypeDefinition {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameters {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameterInfo {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericDeclarationParameter {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl Parse for GenericParameterInfoDefault {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}
