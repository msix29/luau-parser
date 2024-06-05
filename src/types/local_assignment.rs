//! Holding all needed information for local assignments.

use std::sync::Arc;

use super::{Expression, List, NormalizedName, Token};
use crate::generate_derives;

generate_derives! {
    /// A struct holding data for local assignments.
    pub struct LocalAssignment {
        /// The `local` keyword.
        pub local_token: Token,

        /// The List of [`names`](NormalizedName) before the `=` sign.
        pub name_list: List<NormalizedName>,

        /// The `=` sign.
        pub equal_token: Option<Token>,

        /// The list of [`expressions`](Expression) after the `=` sign.
        pub expressions: List<Arc<Expression>>,
    }
}
