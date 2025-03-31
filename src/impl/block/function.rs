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

macro_rules! parse {
    (
        $function_keyword: expr,
        $lexer: ident,
        $errors: ident,
        $name: block
        $(, { $($extra_field:ident),* $(,)?})?
    ) => {{
        let function_keyword = $function_keyword;
        if function_keyword != TokenType::Keyword(Keyword::Function) {
            return None;
        }

        let function_name = $name;

        let generics = parse_bracketed!(
            $lexer,
            $errors,
            "Expected <generic declaration>",
            TokenType::Symbol(Symbol::OpeningAngleBrackets),
            Symbol::ClosingAngleBrackets,
        )
        .map(Pointer::new);

        let parameters = force_parse_bracketed!(
            $lexer,
            $errors,
            "Expected <parameter>",
            (
                TokenType::Symbol(Symbol::OpeningParenthesis),
                TokenType::Symbol(Symbol::OpeningParenthesis)
            ),
            Symbol::ClosingParenthesis,
        );

        maybe_next_token!($lexer, maybe_colon, TokenType::Symbol(Symbol::Colon));
        let return_type = if maybe_colon.is_some() {
            Pointer::<TypeValue>::try_parse($lexer, $errors)
        } else {
            None
        };

        let body = Block::try_parse_with($lexer, $errors, TokenType::Keyword(Keyword::End))
            .unwrap_or_default();

        next_token_recoverable!(
            $lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            $errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Keyword(Keyword::End))
        );

        Some(Self {
            $($($extra_field,)*)?
            function_keyword,
            function_name,
            generics,
            parameters,
            colon: maybe_colon.map(Pointer::new),
            return_type,
            body,
            end_keyword,
        })
    }};
}

impl Parse for LocalFunction {
    fn parse(
        local_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        if local_keyword != TokenType::Keyword(Keyword::Local) {
            return None;
        }

        parse!(
            lexer.next_token(),
            lexer,
            errors,
            {
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
            { local_keyword }
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
        parse!(function_keyword, lexer, errors, {
            GlobalFunctionName::try_parse(lexer, errors).unwrap_or_else(|| {
                GlobalFunctionName::SimpleName(Token::empty(TokenType::Identifier(
                    "*error*".into(),
                )))
            })
        })
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
