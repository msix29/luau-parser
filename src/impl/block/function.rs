//! All `impl` blocks for:
//!
//! * [`LocalFunction`]
//! * [`GlobalFunction`]
//! * [`GlobalFunctionName`]

use luau_lexer::prelude::{Keyword, Lexer, ParseError, PartialKeyword, Symbol, Token, TokenType};

use crate::{
    force_parse_bracketed, parse_bracketed,
    types::{
        Attribute, Block, GetRange, GetRangeError, GlobalFunction, GlobalFunctionName,
        LocalFunction, Parameter, Parse, ParseWithArgs, Pointer, Range, TableAccessKey, TryParse,
        TryParseWithArgs, TypeFunction, TypeValue,
    },
    utils::{get_token_type_display, get_token_type_display_extended},
};

impl Parse for LocalFunction {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let state = lexer.save_state();
        let attributes;
        let local_keyword;

        match token.token_type {
            TokenType::Keyword(Keyword::Local) => {
                attributes = Vec::new();
                local_keyword = token;
            }
            TokenType::Symbol(Symbol::At) => {
                attributes = safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <attribute>",
                    Vec::parse(token, lexer, errors)
                );
                local_keyword = lexer.next_token();
            }
            _ => return None,
        }
        if local_keyword != TokenType::Keyword(Keyword::Local) {
            lexer.set_state(state);

            return None;
        }

        parse_function!(
            attributes,
            lexer.next_token(),
            lexer,
            errors,
            let function_name = {
                next_token_recoverable!(
                    lexer,
                    name,
                    TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                    TokenType::Identifier("*error*".into(),),
                    errors,
                    "Expected ".to_string()
                        + get_token_type_display(&TokenType::Identifier("".into(),))
                );

                name
            },
            { local_keyword, function_name }
        )
    }
}
impl TryParse for LocalFunction {}

impl Parse for GlobalFunctionName {
    fn parse(name: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            name.token_type,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_)
        ) {
            return None;
        }

        maybe_next_token!(
            lexer,
            dot_or_colon,
            TokenType::Symbol(Symbol::Dot) | TokenType::Symbol(Symbol::Colon)
        );
        if let Some(dot_or_colon) = dot_or_colon {
            let is_dot = dot_or_colon == TokenType::Symbol(Symbol::Dot);

            let keys = if is_dot {
                Vec::<TableAccessKey>::parse_with(dot_or_colon.clone(), lexer, errors, false)
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

            let method = if !is_dot {
                next_token_recoverable!(
                    lexer,
                    parsed_method,
                    TokenType::Identifier(_),
                    TokenType::Identifier("*error*".into()),
                    errors,
                    "Expected ".to_string()
                        + get_token_type_display(&TokenType::Identifier("".into()),)
                );

                Some(Pointer::new((dot_or_colon, parsed_method)))
            } else {
                None
            };

            return Some(Self::Table {
                table: name,
                keys,
                method,
            });
        }

        Some(Self::SimpleName(name))
    }
}
impl TryParse for GlobalFunctionName {}

impl Parse for GlobalFunction {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let attributes;
        let function_keyword;

        match token.token_type {
            TokenType::Keyword(Keyword::Function) => {
                attributes = Vec::new();
                function_keyword = token;
            }
            TokenType::Symbol(Symbol::At) => {
                attributes = safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <attribute>",
                    Vec::parse(token, lexer, errors)
                );
                function_keyword = lexer.next_token();
            }
            _ => return None,
        }

        parse_function!(
            attributes,
            function_keyword,
            lexer,
            errors,
            let function_name = {
                GlobalFunctionName::try_parse(lexer, errors).unwrap_or_else(|| {
                    GlobalFunctionName::SimpleName(Token::empty(TokenType::Identifier(
                        "*error*".into(),
                    )))
                })
            },
            { function_name }
        )
    }
}
impl TryParse for GlobalFunction {}

impl Parse for Parameter {
    fn parse(name: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            name.token_type,
            TokenType::Identifier(_)
                | TokenType::PartialKeyword(_)
                | TokenType::Symbol(Symbol::Ellipses)
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

impl Parse for Attribute {
    fn parse(at: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if at != TokenType::Symbol(Symbol::At) {
            return None;
        }

        next_token_recoverable!(
            lexer,
            attribute,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_),
            TokenType::Identifier("*error*".into()),
            errors,
            "Expected ".to_string() + get_token_type_display(&TokenType::Identifier("".into()))
        );

        Some(Self { at, attribute })
    }
}
impl TryParse for Attribute {}

impl GetRange for GlobalFunctionName {
    fn get_range(&self) -> Result<Range, GetRangeError> {
        match self {
            GlobalFunctionName::SimpleName(token) => token.get_range(),
            GlobalFunctionName::Table {
                table,
                keys,
                method,
            } => {
                let table_range = table.get_range();
                let last_range = match method {
                    Some(method) => method.1.get_range(),
                    None => keys.get_range(),
                };

                if let Ok(last_range) = last_range {
                    Ok(Range::new(table_range?.start, last_range.end))
                } else {
                    table_range
                }
            }
        }
    }
}

impl Parse for TypeFunction {
    fn parse(
        token_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        let state = lexer.save_state();

        let export_keyword = if token_keyword == TokenType::PartialKeyword(PartialKeyword::Export) {
            let temp = token_keyword;
            token_keyword = lexer.next_token();

            Some(temp)
        } else {
            None
        };

        let function_keyword = lexer.next_token();

        if token_keyword != TokenType::PartialKeyword(PartialKeyword::Type)
            || function_keyword != TokenType::Keyword(Keyword::Function)
        {
            lexer.set_state(state);

            return None;
        }

        parse_function!(
            function_keyword,
            lexer,
            errors,
            let function_name = {
                next_token_recoverable!(
                    lexer,
                    name,
                    TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                    TokenType::Identifier("*error*".into(),),
                    errors,
                    "Expected ".to_string()
                        + get_token_type_display(&TokenType::Identifier("".into(),))
                );

                name
            },
            { export_keyword, type_keyword, function_name }
        )
    }
}
impl TryParse for TypeFunction {}
