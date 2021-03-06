//! ARM® Cortex®-M platform crate for Drone, an Embedded Operating System.
//!
//! # Supported Cores
//!
//! | Architecture | Core name              | Rust target                 | `cortex_m_core` config flag |
//! |--------------|------------------------|-----------------------------|-----------------------------|
//! | ARMv7-M      | ARM® Cortex®-M3 r0p0   | `thumbv7m-none-eabi`        | `cortex_m3_r0p0`            |
//! | ARMv7-M      | ARM® Cortex®-M3 r1p0   | `thumbv7m-none-eabi`        | `cortex_m3_r1p0`            |
//! | ARMv7-M      | ARM® Cortex®-M3 r1p1   | `thumbv7m-none-eabi`        | `cortex_m3_r1p1`            |
//! | ARMv7-M      | ARM® Cortex®-M3 r2p0   | `thumbv7m-none-eabi`        | `cortex_m3_r2p0`            |
//! | ARMv7-M      | ARM® Cortex®-M3 r2p1   | `thumbv7m-none-eabi`        | `cortex_m3_r2p1`            |
//! | ARMv7E-M     | ARM® Cortex®-M4 r0p0   | `thumbv7em-none-eabi`       | `cortex_m4_r0p0`            |
//! | ARMv7E-M     | ARM® Cortex®-M4 r0p1   | `thumbv7em-none-eabi`       | `cortex_m4_r0p1`            |
//! | ARMv7E-M     | ARM® Cortex®-M4F r0p0  | `thumbv7em-none-eabihf`     | `cortex_m4f_r0p0`           |
//! | ARMv7E-M     | ARM® Cortex®-M4F r0p1  | `thumbv7em-none-eabihf`     | `cortex_m4f_r0p1`           |
//! | ARMv8-M      | ARM® Cortex®-M33 r0p2  | `thumbv8m.main-none-eabi`   | `cortex_m33_r0p2`           |
//! | ARMv8-M      | ARM® Cortex®-M33 r0p3  | `thumbv8m.main-none-eabi`   | `cortex_m33_r0p3`           |
//! | ARMv8-M      | ARM® Cortex®-M33 r0p4  | `thumbv8m.main-none-eabi`   | `cortex_m33_r0p4`           |
//! | ARMv8-M      | ARM® Cortex®-M33F r0p2 | `thumbv8m.main-none-eabihf` | `cortex_m33f_r0p2`          |
//! | ARMv8-M      | ARM® Cortex®-M33F r0p3 | `thumbv8m.main-none-eabihf` | `cortex_m33f_r0p3`          |
//! | ARMv8-M      | ARM® Cortex®-M33F r0p4 | `thumbv8m.main-none-eabihf` | `cortex_m33f_r0p4`          |
//!
//! Rust target triple and `cortex_m_core` config flag should be set at the
//! application level according to this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-cortex-m/0.12/)
//!
//! # Usage
//!
//! Place the following to the Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! drone-cortex-m = { version = "0.12.0", features = [...] }
//! ```

#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exhaustive_patterns)]
#![feature(lang_items)]
#![feature(marker_trait_attr)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(never_type_fallback)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(untagged_unions)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::precedence,
    clippy::shadow_unrelated,
    clippy::type_repetition_in_bounds
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod drv;
pub mod fib;
pub mod itm;
pub mod map;
pub mod prelude;
pub mod proc_loop;
pub mod processor;
pub mod reg;
pub mod sv;
pub mod thr;

#[cfg(not(feature = "std"))]
mod lang_items;

mod drone_core_macro_reexport {
    pub use drone_core::{reg, thr};
}

pub use drone_core_macro_reexport::*;

/// Defines the supervisor type.
///
/// See [the module level documentation](sv) for details.
#[doc(inline)]
pub use drone_cortex_m_macros::sv;

#[prelude_import]
#[allow(unused_imports)]
use crate::prelude::*;
