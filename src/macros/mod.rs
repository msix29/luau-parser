//! Helpful macros for the lsp

mod parse;

/// Handles [`TokenType::Error`](luau_lexer::prelude::TokenType::Error).
///
/// # Usage
///
/// ```ignore
/// handle_error_token!(errors, <expr>)
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! handle_error_token {
    ($errors: ident, $error: expr) => {{
        $errors.push($error);

        None
    }};
}

/// Safely `unwrap` an [`Option`] by using the default value and sending the
/// passed error if it's [`None`].
///
/// # Usage
///
/// ```ignore
/// safe_unwrap!(
///     lexer,
///     errors,
///     "<error message>",
///     <expr> // this must be an `Option`, or have an `unwrap_or_else` function.
/// )
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! safe_unwrap {
    ($lexer: ident, $errors: ident, $error_message: expr, $expr: expr) => {{
        $expr.unwrap_or_else(|| {
            let state = $lexer.save_state();
            $errors.push(ParseError::new(
                state.lexer_position(),
                $error_message,
                Some(state.lexer_position()),
            ));

            Default::default()
        })
    }};
}
