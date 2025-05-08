//! # Types module
//!
//! This just reexports the struct and traits for easier importing.
//!
//! ## Note
//!
//! This file only contains the definitions for items, for actual implementations,
//! check the files under `src/impl`. Each type will have it's implementation in
//! the same place, ex. types in `types/value/function.rs` will have
//! their implementations in `impl/value/function.rs`. The only `impl`
//! here is for `Statement`, and it isn't exposed directly to consumers of this
//! crate, but rather through other functions.

#[cfg(not(feature = "async"))]
use std::rc::Rc as PointerInner;
#[cfg(feature = "async")]
use std::sync::Arc as PointerInner;

/// A helper macro to reexport modules.
macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*
        $( pub use $name::*; )*
    };
}

reexport!(
    block, bracketed, cst, expression, list, literals, name, traits, value
);

/// The main pointer used in the [`Cst`]. It's just [`Rc`](std::rc::Rc)
/// (or [`Arc`](std::sync::Arc) if "async" feature is enabled). The only reason
/// this type exists is to allow easily switching to other pointer types, by only
/// editing one line instead of mass refactoring, and without breaking crates that
/// use it.
pub type Pointer<T> = PointerInner<T>;

/// An enum representing printing errors that stopped [`Cst::try_print`] from working.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PrintingError {
    /// The [`CST`](Cst) has syntax errors.
    ErroneousCst,
}
