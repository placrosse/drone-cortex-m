//! Memory-mapped registers.

pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;

pub use self::atomic::*;
pub use self::bit_band::*;

#[allow(clippy::doc_markdown)]
mod map {
  use drone_core::reg::map;
  use reg::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_map.rs"));
}

pub use self::map::*;

#[allow(clippy::doc_markdown)]
mod tokens {
  use drone_core::reg::tokens;
  use reg::map::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_tokens.rs"));
}

pub use self::tokens::*;
