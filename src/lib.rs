//! Integer conversions
//!
//! This crate provides explicit conversions between integer types.
//!
//! # Features
//!
//! - [`ZeroExtend`] / [`SignExtend`] / [`Extend`]: Extend from a smaller to larger integer.
//! - [`Signed`] / [`IsSigned`] / [`IsUnsigned`]: Interchange between signed and unsigned types.
//! - [`Truncate`]: Truncate integers.
//! - [`Split`] / [`Join`]: Split integers in half and joins them back together.
//!
//! Various helpers are also provided to be used with the turbofish syntax (`::<>`).

// Features
#![no_std]
// Lints
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery)]
// We turn off warnings we don't need
#![allow(clippy::blanket_clippy_restriction_lints)]
// Due to the way we organize modules, this happens, but we can't remove the suffix
#![allow(clippy::module_name_repetitions)]
// We prefer implicit returns
#![allow(clippy::implicit_return)]
// We need integer arithmetic to compute type sizes
#![allow(clippy::integer_arithmetic)]
// We want to explicitly use the type we're converting to in implementations
#![allow(clippy::use_self)]
// We use integer division when we want to discard any decimal parts
#![allow(clippy::integer_division)]
// In tests, we make sure `as` conversions are correct.
#![cfg_attr(test, allow(clippy::as_conversions))]
// Tests sometimes contain a lot of cases, but they're all simple
#![cfg_attr(test, allow(clippy::cognitive_complexity))]

// Modules
pub mod extend;
pub mod sign;
pub mod split;
pub mod trunc;

// Exports
pub use extend::{Extend, Extended, SignExtend, SignExtended, ZeroExtend, ZeroExtended};
pub use sign::{IsSigned, IsUnsigned, Signed};
pub use split::{Join, Split};
pub use trunc::{Truncate, Truncated};
