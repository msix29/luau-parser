use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};
use std::cell::Cell;

use crate::{
    safe_unwrap,
    types::{
        Bracketed, BracketedList, Expression, FunctionArguments, GetRange, GetRangeError, Parse,
        ParseWithArgs, Pointer, Range, Table, TableAccessKey, TableField, TableFieldValue,
        TableKey, TryParse, TryParseWithArgs, TypeValue,
    },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ParseArgs {
    is_type: bool,
    inferred_keys: Cell<u32>,
}
impl ParseArgs {
    #[inline]
    fn new(is_type: bool, inferred_keys: u32) -> Self {
        Self {
            is_type,
            inferred_keys: Cell::new(inferred_keys),
        }
    }
}

impl TableKey {
    #[inline]
    fn undefined_number(parse_args: &ParseArgs) -> Self {
        Self::UndefinedNumber(
            parse_args
                .inferred_keys
                .replace(parse_args.inferred_keys.get() + 1),
        )
    }

    #[inline]
    fn undefined_string() -> Self {
        Self::UndefinedString("number".into())
    }
}

impl ParseWithArgs<bool> for TableKey {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        match token.token_type {
            TokenType::Identifier(_) | TokenType::PartialKeyword(_) => Some(Self::Simple(token)),
            TokenType::Symbol(Symbol::OpeningBrackets) => {
                if is_type {
                    Bracketed::<_>::parse_with(
                        token,
                        lexer,
                        errors,
                        ("Expected <type>", Symbol::ClosingBrackets),
                    )
                    .map(Self::Type)
                } else {
                    Bracketed::<_>::parse_with(
                        token,
                        lexer,
                        errors,
                        ("Expected <expr>", Symbol::ClosingBrackets),
                    )
                    .map(Self::Expression)
                }
            }
            _ => None,
        }
    }
}

impl ParseWithArgs<&ParseArgs> for TableField {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        parse_args: &ParseArgs,
    ) -> Option<Self> {
        if token == TokenType::Symbol(Symbol::ClosingCurlyBrackets) {
            // Sometimes causes issues when the last item in the table is trailing
            // this just ensures it never happens.
            return None;
        }

        let state = lexer.save_state();

        let (key, equal_or_colon) = if let Some(key) =
            TableKey::parse_with(token.clone(), lexer, errors, parse_args.is_type)
        {
            let equal_or_colon = if parse_args.is_type {
                maybe_next_token!(lexer, temp, TokenType::Symbol(Symbol::Colon));

                temp
            } else {
                maybe_next_token!(lexer, temp, TokenType::Symbol(Symbol::Equal));

                temp
            };

            (Some(Pointer::new(key)), equal_or_colon)
        } else {
            (None, None)
        };

        if key.is_none() || equal_or_colon.is_none() {
            lexer.set_state(state);

            return Some(Self {
                key: if parse_args.is_type {
                    Pointer::new(TableKey::undefined_string())
                } else {
                    Pointer::new(TableKey::undefined_number(parse_args))
                },
                equal_or_colon: None,
                value: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <type>",
                    TableFieldValue::parse_with(token.clone(), lexer, errors, parse_args.is_type)
                        .map(Pointer::new)
                ),
            });
        }

        let key = key.unwrap();

        let value = Pointer::new(TableFieldValue::try_parse_with(
            lexer,
            errors,
            parse_args.is_type,
        )?);

        Some(Self {
            key,
            equal_or_colon,
            value,
        })
    }
}

impl ParseWithArgs<bool> for TableFieldValue {
    #[inline]
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        if is_type {
            TypeValue::parse(token, lexer, errors).map(Self::Type)
        } else {
            Expression::parse(token, lexer, errors).map(Self::Expression)
        }
    }
}

impl ParseWithArgs<bool> for Table {
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        is_type: bool,
    ) -> Option<Self> {
        if !matches!(
            token.token_type,
            TokenType::Symbol(Symbol::OpeningCurlyBrackets)
        ) {
            return None;
        }

        BracketedList::<TableField>::parse_with(
            token,
            lexer,
            errors,
            (
                "Expected <table-field>",
                Symbol::ClosingCurlyBrackets,
                &ParseArgs::new(is_type, 1),
            ),
        )
        .map(Self)
    }
}

impl Parse<FunctionArguments> for Table {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<FunctionArguments> {
        Self::parse_with(token, lexer, errors, false).map(FunctionArguments::Table)
    }
}
impl TryParse<FunctionArguments> for Table {}

impl Parse<TableAccessKey> for TableKey {
    #[inline]
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<TableAccessKey> {
        Self::parse_with(token, lexer, errors, false)
            .map(Pointer::new)
            .map(TableAccessKey::Expression)
    }
}
impl TryParse<TableAccessKey> for TableKey {}

impl GetRange for TableKey {
    #[inline]
    fn get_range(&self) -> Result<Range, GetRangeError> {
        match self {
            TableKey::ERROR => Err(GetRangeError::ErrorVariant),
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => {
                Err(GetRangeError::UndefinedKey)
            }
            TableKey::Simple(token) => token.get_range(),
            TableKey::Expression(bracketed) => bracketed.get_range(),
            TableKey::Type(bracketed) => bracketed.get_range(),
        }
    }
}

impl GetRange for TableField {
    #[inline]
    fn get_range(&self) -> Result<Range, GetRangeError> {
        let value_range = self.value.get_range();

        if let Ok(key_range) = self.key.get_range() {
            Ok(Range::new(key_range.start, value_range?.end))
        } else {
            value_range
        }
    }
}
