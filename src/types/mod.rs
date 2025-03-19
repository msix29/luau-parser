//! # Types module
//!
//! This just rexports the struct and traits for easier importing.
//!
//! ## Note
//!
//! This file only contains the definitions for items, for actual implementations,
//! check the files under `src/impl`. Each type will have it's implementation in
//! the same place, ex. types in `types/value/function.rs` will have
//! their implementations in `impl/value/function.rs`, same thing for display
//! implementations but they'll be in `src/display` instead.

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*
        $( pub use $name::*; )*
    };
}

reexport!(
    block,
    cst,
    expression,
    function,
    list,
    literals,
    name,
    range,
    traits,
    value,
);

/// An enum representing printing errors that stopped [`Cst::try_print`] from working.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PrintingError {
    /// The [`CST`](Cst) has syntax errors.
    IncompleteAst,
}
