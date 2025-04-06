use luau_lexer::prelude::{Lexer, Token, TokenType};
use smol_str::SmolStr;

use crate::types::{AstStatus, Block, Cst, ParseWithArgs, Print, PrintingError};

impl Cst {
    pub(crate) fn parse<T: Into<SmolStr>>(token: Token, lexer: &mut Lexer, uri: T) -> Self {
        let mut errors = Vec::new();

        if token == TokenType::EndOfFile {
            return Self {
                uri: uri.into(),
                block: Block::default(),
                errors,
                status: AstStatus::Complete,
            };
        }

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

    #[inline]
    pub fn has_errors(&self) -> bool {
        //FIXME:
        // Check this or `self.errors`? The result will be the same as long
        // as the users don't edit the CST themselves.
        self.status == AstStatus::HasErrors
    }

    pub fn try_print(&self) -> Result<String, PrintingError> {
        if self.has_errors() {
            Err(PrintingError::ErroneousCst)
        } else {
            Ok(self.block.print())
        }
    }
}
