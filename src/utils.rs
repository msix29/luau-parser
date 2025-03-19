use luau_lexer::prelude::{CompoundOperator, Operator, Symbol, TokenType};

pub(crate) fn get_token_type_display(token_type: &TokenType) -> String {
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
    }
    .to_string()
}

pub(crate) fn get_token_type_display_extended(token_type: &TokenType) -> String {
    match token_type {
        TokenType::Keyword(_) => "<keyword>",
        TokenType::PartialKeyword(_) => "<partial keyword>",
        TokenType::Identifier(_) => "<identifier>",
        TokenType::Literal(_) => "<expression>",
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
        },
        TokenType::Operator(operator) => match operator {
            Operator::Plus => "`+`",
            Operator::Minus => "`-`",
            Operator::Division => "`/`",
            Operator::FloorDivision => "`//`",
            Operator::Multiplication => "`*`",
            Operator::Modulo => "`%`",
            Operator::Exponentiation => "`^`",
            Operator::Equal => "`=`",
            Operator::Length => "`#`",
            Operator::And => "`and`",
            Operator::Or => "`or`",
            Operator::Not => "`not`",
            Operator::Concatenation => "`..`",
            Operator::NotEqual => "`~=`",
            Operator::Intersection => "`&`",
            Operator::Union => "`|`",
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
    }
    .to_string()
}
