//! # Types module
//!
//! This just rexports the struct and traits for easier importing.
//!
//! ## Note
//!
//! This file only contains the definitions for items, for actual implementations,
//! check the files under `src/ast`. Each type will have it's implementation in
//! the same place, ex. types in `shared_types/value/function.rs` will have
//! their implementations in `ast/value/function.rs`, same thing for display
//! implementations but they'll be in `src/display` instead.

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*
        $( pub use $name::*; )*
    };
}

reexport!(
    block,
    expression,
    function,
    list,
    literals,
    local_assignment,
    name,
    position,
    range,
    set_expressions,
    statement,
    token,
    traits,
    type_definition,
    value,
);
