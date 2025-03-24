//! The `parse_bracketed` macro.

#[macro_export]
macro_rules! parse_bracketed {
    (
        $lexer: ident,
        $errors: ident,
        $error_message: literal,
        $opening: pat,
        $closing: expr,
    ) => {{
        $crate::maybe_next_token!($lexer, __opening_bracket, $opening);

        if let Some(__opening_bracket) = __opening_bracket {
            $crate::types::Bracketed::<_>::parse_with(
                __opening_bracket,
                $lexer,
                $errors,
                ($error_message, $closing),
            )
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! force_parse_bracketed {
    (
        $lexer: ident,
        $errors: ident,
        $error_message: literal,
        ($opening: pat, $opening_replacement: expr),
        $closing: expr,
    ) => {{
        $crate::next_token_recoverable!(
            $lexer,
            __opening_bracket,
            $opening,
            $opening_replacement,
            $errors,
            $error_message
        );

        $crate::types::Bracketed::<_>::parse_with(
            __opening_bracket,
            $lexer,
            $errors,
            ($error_message, $closing),
        )
        .unwrap_or_else(|| $crate::types::Bracketed {
            opening_bracket: Token::empty($opening_replacement),
            item: Default::default(),
            closing_bracket: Token::empty($closing.into()),
        })
    }};
}

#[macro_export]
macro_rules! safe_unwrap {
    ($lexer: ident, $errors: ident, $error_message: literal, $expr: expr) => {{
        $expr.unwrap_or_else(|| {
            let state = $lexer.save_state();
            $errors.push(ParseError::new(
                state.lexer_position(),
                $error_message.to_string(),
                Some(state.lexer_position()),
            ));

            Default::default()
        })
    }};
}
