//! All `impl` blocks for [`Cst`].

use luau_lexer::prelude::{Lexer, Token, TokenType};
use smol_str::SmolStr;

use crate::types::{AstStatus, Block, Cst, ParseWithArgs, Print, PrintingError};

impl Cst {
    /// The actual parsing logic for the [`Cst`].
    pub(crate) fn parse<T: Into<SmolStr>>(token: Token, lexer: &mut Lexer, uri: T) -> Self {
        let mut errors = Vec::new();

        let block = Block::parse_with(token, lexer, &mut errors, None::<Token>);
        let status = if errors.is_empty() {
            AstStatus::Complete
        } else {
            AstStatus::HasErrors
        };

        Self {
            uri: uri.into(),
            block: block.unwrap_or_default(),
            errors,
            status,
        }
    }

    /// Whether or not this [`Cst`] has errors.
    #[inline]
    pub fn has_errors(&self) -> bool {
        //FIXME:
        // Check this or `self.errors`? The result will be the same as long
        // as the users don't edit the CST themselves.
        self.status == AstStatus::HasErrors
    }

    /// Try printing the [`Cst`] back into source code. This'll only fail if
    /// [`Cst.has_errors()`](Self::has_errors()) returns `true`.
    pub fn try_print(&self) -> Result<String, PrintingError> {
        if self.has_errors() {
            Err(PrintingError::ErroneousCst)
        } else {
            Ok(self.block.print())
        }
    }
}
