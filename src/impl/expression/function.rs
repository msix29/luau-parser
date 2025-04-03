use luau_lexer::prelude::{Keyword, Lexer, Literal, ParseError, Symbol, Token, TokenType};

use crate::{
    force_parse_bracketed, parse_bracketed,
    types::{
        Block, BracketedList, Closure, FunctionArguments, FunctionCall, FunctionCallInvoked, Parse,
        ParseWithArgs, Pointer, PrefixExp, Table, TableAccessPrefix, TryParse, TryParseWithArgs,
        TypeValue,
    },
    utils::get_token_type_display_extended,
};

impl Parse for FunctionCallInvoked {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let prefix_exp = Pointer::new(PrefixExp::parse(token, lexer, errors)?);
        let state = lexer.save_state();

        let maybe_colon = lexer.next_token();

        if maybe_colon != TokenType::Symbol(Symbol::Colon) {
            lexer.set_state(state);

            return Some(Self::Function(prefix_exp));
        }

        Some(Self::TableMethod {
            table: prefix_exp,
            colon: Pointer::new(maybe_colon),
            method: Pointer::new(lexer.next_token()),
        })
    }
}
impl TryParse for FunctionCallInvoked {}

impl Parse for FunctionCall {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Some(Self {
            invoked: FunctionCallInvoked::parse(token, lexer, errors)?,
            arguments: FunctionArguments::try_parse(lexer, errors)?,
        })
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

impl Parse for Closure {
    fn parse(
        function_keyword: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        parse_function!(function_keyword, lexer, errors)
    }
}
