//! All `impl` blocks for function call-related types:
//!
//! * [`FunctionCallInvoked`]
//! * [`FunctionCall`]
//! * [`FunctionArguments`]
//! * [`FunctionArgument`]
//! * [`Closure`]

use luau_lexer::prelude::{Keyword, Lexer, Literal, ParseError, Symbol, Token, TokenType};

use crate::{
    force_parse_bracketed, parse_bracketed,
    types::{
        Block, BracketedList, Closure, Expression, FunctionArgument, FunctionArguments,
        FunctionCall, FunctionCallInvoked, Parse, ParseWithArgs, Pointer, PrefixExp, Table,
        TableAccessPrefix, TryParse, TryParseWithArgs, TypeValue,
    },
    utils::{get_token_type_display, get_token_type_display_extended},
};

impl Parse for FunctionCallInvoked {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let prefix_exp = Pointer::new(PrefixExp::parse(token, lexer, errors)?);

        maybe_next_token!(lexer, colon, TokenType::Symbol(Symbol::Colon));
        let Some(colon) = colon else {
            return Some(Self::Function(prefix_exp));
        };

        next_token_recoverable!(
            lexer,
            method,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_),
            TokenType::Identifier("*error*".into(),),
            errors,
            "Expected ".to_string() + get_token_type_display(&TokenType::Identifier("".into(),))
        );

        Some(Self::TableMethod {
            table: prefix_exp,
            colon: Pointer::new(colon),
            method: Pointer::new(method),
        })
    }
}
impl TryParse for FunctionCallInvoked {}

impl FunctionCall {
    /// Parse a [`FunctionCall`] with the passed [`FunctionCallInvoked`]. This
    /// method keeps parsing repeatedly to allow for chained of functions calls:
    ///
    /// ```lua
    /// foo()()()
    /// ```
    pub fn try_parse_with_invoked(
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        mut invoked: FunctionCallInvoked,
    ) -> Option<Self> {
        let mut found_arguments = false;
        while let Some(arguments) = FunctionArguments::try_parse(lexer, errors) {
            found_arguments = true;
            invoked = FunctionCallInvoked::Function(Pointer::new(PrefixExp::FunctionCall(Self {
                invoked,
                arguments,
            })));
        }

        // Avoid infinite recursions.
        if !found_arguments {
            return None;
        }

        if let FunctionCallInvoked::Function(pointer) = invoked {
            if let PrefixExp::FunctionCall(function_call) = (*pointer).clone() {
                return Some(function_call);
            }
        }

        None
    }
}

impl Parse for FunctionCall {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let invoked = FunctionCallInvoked::parse(token, lexer, errors)?;
        let arguments = FunctionArguments::try_parse(lexer, errors);

        if let Some(arguments) = arguments {
            return Some(FunctionCall { invoked, arguments });
        }
        if let FunctionCallInvoked::Function(pointer) = invoked {
            if let PrefixExp::FunctionCall(call) = (*pointer).clone() {
                return Some(call);
            }
        }

        None
    }
}
impl TryParse for FunctionCall {
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            invoked: FunctionCallInvoked::try_parse(lexer, errors)?,
            arguments: FunctionArguments::try_parse(lexer, errors)?,
        })
    }
}

impl Parse<PrefixExp> for FunctionCall {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<PrefixExp> {
        Self::parse(token, lexer, errors).map(PrefixExp::FunctionCall)
    }
}
impl Parse<TableAccessPrefix> for FunctionCall {
    fn parse(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<TableAccessPrefix> {
        Pointer::<Self>::parse(token, lexer, errors).map(TableAccessPrefix::FunctionCall)
    }
}

impl Parse for FunctionArguments {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if matches!(token.token_type, TokenType::Literal(Literal::String(_))) {
            return Some(Self::String(token));
        }
        if token.token_type == TokenType::Symbol(Symbol::OpeningParenthesis) {
            return BracketedList::parse_with(
                token,
                lexer,
                errors,
                ("Expected <expr>", Symbol::ClosingParenthesis),
            )
            .map(Self::List);
        }

        Table::parse(token.clone(), lexer, errors)
    }
}
impl TryParse for FunctionArguments {}

impl Parse for FunctionArgument {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if matches!(token.token_type, TokenType::Symbol(Symbol::Ellipses)) {
            Some(Self::VariadicValues(token))
        } else {
            Expression::parse(token, lexer, errors).map(Self::Expression)
        }
    }
}
impl TryParse for FunctionArgument {}

impl Parse for Closure {
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
            let attributes = attributes;
            function_keyword,
            lexer,
            errors,
            { attributes }
        )
    }
}
