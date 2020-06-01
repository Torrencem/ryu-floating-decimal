//! This crate copies the internals of the [ryu rust crate](https://github.com/dtolnay/ryu), exposing some useful functions and types for more flexible float printing.
//! This crate exposes the functions `d2d` and `f2d`, which convert from `f64` to `FloatingDecimal64` and `f32` to `FloatingDecimal32` respectively. These floating decimals can be converted to strings in a custom way.

// License note: this main file is mostly unchanged, except for removing modules that
// are no longer exported, and adding the exposed d2d and f2d functions. This module
// is similarly provided under the terms of the Apache License, Version 2.0.

#![no_std]
#![doc(html_root_url = "https://docs.rs/ryu-floating-decimal/1.0.4")]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(cast_lossless, many_single_char_names, unreadable_literal,)
)]

#[cfg(feature = "no-panic")]
extern crate no_panic;

mod common;
mod d2s;
#[cfg(not(feature = "small"))]
mod d2s_full_table;
mod d2s_intrinsics;
#[cfg(feature = "small")]
mod d2s_small_table;
mod f2s;

#[cfg(feature = "no-panic")]
use no_panic::no_panic;

pub use d2s::FloatingDecimal64;
use core::mem;

/// Convert an `f64` to a floating decimal using the ryu algorithm
///
/// # Example
///
/// ```
/// use ryu_floating_decimal::d2d;
/// let value: f64 = 3.14;
/// let decimal = d2d(value);
/// assert_eq!(decimal.mantissa, 314);
/// assert_eq!(decimal.exponent, -2);
/// ```
#[cfg_attr(feature = "no-panic", inline)]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn d2d(val: f64) -> FloatingDecimal64 {
    let bits: u64 = unsafe { mem::transmute(val) };
    let ieee_mantissa = bits & ((1u64 << d2s::DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> d2s::DOUBLE_MANTISSA_BITS) as u32 & ((1u32 << d2s::DOUBLE_EXPONENT_BITS) - 1);
    d2s::d2d(ieee_mantissa, ieee_exponent)
}

pub use f2s::FloatingDecimal32;

/// Convert an `f32` to a floating decimal using the ryu algorithm
///
/// # Example
///
/// ```
/// use ryu_floating_decimal::f2d;
/// let value: f32 = 12.091;
/// let decimal = f2d(value);
/// assert_eq!(decimal.mantissa, 12091);
/// assert_eq!(decimal.exponent, -3);
/// ```
#[cfg_attr(feature = "no-panic", inline)]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn f2d(val: f32) -> FloatingDecimal32 {
    let bits: u32 = unsafe { mem::transmute(val) };
    let ieee_mantissa = bits & ((1u32 << f2s::FLOAT_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> f2s::FLOAT_MANTISSA_BITS) as u32 & ((1u32 << f2s::FLOAT_EXPONENT_BITS) - 1);
    f2s::f2d(ieee_mantissa, ieee_exponent)
}
