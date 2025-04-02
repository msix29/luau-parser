use luau_lexer::prelude::{Keyword, Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    force_parse_bracketed, parse_bracketed,
    prelude::{GetRangeError, Range},
    types::{
        Block, GetRange, GlobalFunction, GlobalFunctionName, LocalFunction, Parse, ParseWithArgs,
        Pointer, TableAccessKey, TryParse, TryParseWithArgs, TypeValue,
    },
    utils::{get_token_type_display, get_token_type_display_extended},
};

impl Parse for LocalFunction {
    fn parse(
        local_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if local_keyword != TokenType::Keyword(Keyword::Local) {
            return None;
        }

        parse_function!(
            lexer.next_token(),
            lexer,
            errors,
            let function_name = {
                next_token_recoverable!(
                    lexer,
                    name,
                    TokenType::Identifier(_),
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
        if !matches!(name.token_type, TokenType::Identifier(_)) {
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
                Vec::<TableAccessKey>::try_parse_with(lexer, errors, false).unwrap_or_default()
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
    fn parse(
        function_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        parse_function!(
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
