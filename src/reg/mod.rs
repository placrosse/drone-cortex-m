//! Memory-mapped registers.

pub mod prelude;

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod bindings;
#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod bit_band;
#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod mappings;
#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod shared;

pub use self::bindings::*;
pub use self::bit_band::*;
pub use self::mappings::*;
pub use self::shared::*;
