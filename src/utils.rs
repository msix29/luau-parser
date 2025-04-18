//! Utility functions for this crate.

use luau_lexer::prelude::{
    CompoundOperator, Keyword, Literal, Operator, PartialKeyword, Symbol, TokenType,
};

// Optimization trick
// The functions here should all be `O(1)`, making them return `String`
// will make them `O(n)` due to the heap allocation, and thus they return
// `&str`. This small difference makes both functions at least 10x faster!
// And since the functions are only used internally (in `format!`, for errors),
// this edit has no side effects.

/// Display a high-level overview of the passed [`token type`](TokenType) as a
/// string. Unlike [`get_token_type_display_extended`], this doesn't differentiate
/// between items of the same enum, ie. all items in [`Symbol`] will have the same
/// output.
pub(crate) fn get_token_type_display(token_type: &TokenType) -> &str {
    match token_type {
        TokenType::Keyword(_) => "<keyword>",
        TokenType::PartialKeyword(_) => "<partial keyword>",
        TokenType::Identifier(_) => "<identifier>",
        TokenType::Literal(_) => "<expression>",
        TokenType::Symbol(_) => "<symbol>",
        TokenType::Operator(_) => "<operator>",
        TokenType::CompoundOperator(_) => "<compound operator>",
        TokenType::EndOfFile => "<eof>",
        TokenType::Error(_) => "<error>",
        TokenType::Comment(_) => "<comment>",
    }
}

/// Display the passed [`token type`](TokenType) as a string. Unlike
/// [`get_token_type_display`], this differentiates between items of the same enum,
/// ie. [`Symbol::Colon`] and [`Symbol::Equal`] will have different outputs.
pub(crate) fn get_token_type_display_extended(token_type: &TokenType) -> &str {
    match token_type {
        TokenType::Keyword(keyword) => match keyword {
            Keyword::Local => "<local>",
            Keyword::Function => "<function>",
            Keyword::If => "<if>",
            Keyword::Elseif => "<elseif>",
            Keyword::Then => "<then>",
            Keyword::Else => "<else>",
            Keyword::While => "<while>",
            Keyword::For => "<for>",
            Keyword::In => "<in>",
            Keyword::Do => "<do>",
            Keyword::Break => "<break>",
            Keyword::Return => "<return>",
            Keyword::End => "<end>",
            Keyword::Repeat => "<repeat>",
            Keyword::Until => "<until>",
            Keyword::Nil => "<nil>",
        },
        TokenType::PartialKeyword(partial_keyword) => match partial_keyword {
            PartialKeyword::Type => "<type>",
            PartialKeyword::Continue => "<continue>",
            PartialKeyword::Export => "<export>",
            PartialKeyword::TypeOf => "<typeof>",
        },
        TokenType::Identifier(_) => "<identifier>",
        TokenType::Literal(literal) => match literal {
            Literal::Number(_) => "<number>",
            Literal::String(_) => "<string>",
            Literal::Boolean(_) => "<boolean>",
        },
        TokenType::Symbol(symbol) => match symbol {
            Symbol::OpeningCurlyBrackets => "<opening curly brackets>",
            Symbol::ClosingCurlyBrackets => "<closing curly brackets>",
            Symbol::OpeningBrackets => "<opening brackets>",
            Symbol::ClosingBrackets => "<closing brackets>",
            Symbol::OpeningParenthesis => "<opening parenthesis>",
            Symbol::ClosingParenthesis => "<closing parenthesis>",
            Symbol::Semicolon => "<semicolon>",
            Symbol::Colon => "<colon>",
            Symbol::Dot => "<dot>",
            Symbol::Comma => "<comma>",
            Symbol::Ellipses => "<ellipses>",
            Symbol::OpeningAngleBrackets => "<opening angle brackets>",
            Symbol::ClosingAngleBrackets => "<closing angle brackets>",
            Symbol::Arrow => "<arrow>",
            Symbol::Typecast => "<typecast>",
            Symbol::Equal => "`=`",
            Symbol::At => "`@`",
        },
        TokenType::Operator(operator) => match operator {
            Operator::Plus => "`+`",
            Operator::Minus => "`-`",
            Operator::Division => "`/`",
            Operator::FloorDivision => "`//`",
            Operator::Multiplication => "`*`",
            Operator::Modulo => "`%`",
            Operator::Exponentiation => "`^`",
            Operator::Length => "`#`",
            Operator::And => "`and`",
            Operator::Or => "`or`",
            Operator::Not => "`not`",
            Operator::Concatenation => "`..`",
            Operator::NotEqual => "`~=`",
            Operator::Intersection => "`&`",
            Operator::Union => "`|`",
            Operator::Optional => "`?`",
        },
        TokenType::CompoundOperator(compound_operator) => match compound_operator {
            CompoundOperator::PlusEqual => "`+=`",
            CompoundOperator::MinusEqual => "`-=`",
            CompoundOperator::DivisionEqual => "`/=`",
            CompoundOperator::FloorDivisionEqual => "`//=`",
            CompoundOperator::MultiplicationEqual => "`*=`",
            CompoundOperator::ModuloEqual => "`%=`",
            CompoundOperator::ExponentiationEqual => "`^=`",
            CompoundOperator::EqualEqual => "`==`",
            CompoundOperator::ConcatenationEqual => "`..=`",
            CompoundOperator::LessThanOrEqualTo => "`<=`",
            CompoundOperator::GreaterThanOrEqualTo => "`>=`",
        },
        TokenType::EndOfFile => "<eof>",
        TokenType::Error(_) => "<error>",
        TokenType::Comment(_) => "<comment>",
    }
}
