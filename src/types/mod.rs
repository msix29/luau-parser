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
//! implementations but they'll be in `src/display` instead. The only `impl`
//! here is for `Statement`, and it isn't exposed directly to consumers of this
//! crate but rather through other functions.

use std::rc::Rc;

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*
        $( pub use $name::*; )*
    };
}

reexport!(block, bracketed, cst, expression, list, literals, name, range, traits, value);

/// The main pointer used in the [`Cst`]. It's just [`Rc`]. The only reason
/// this type exists is to allow easily switching to others, like [`Box`] or
/// [`Arc`](std::sync::Arc) by only editing one line instead of mass refactoring.
pub type Pointer<T> = Rc<T>;

/// An enum representing printing errors that stopped [`Cst::try_print`] from working.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PrintingError {
    /// The [`CST`](Cst) has syntax errors.
    ErroneousCst,
}
