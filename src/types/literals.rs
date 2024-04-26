//! Module holding types for literals.

use crate::prelude::SingleToken;

/// A struct represnting a string literal.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringLiteral(pub SingleToken);
