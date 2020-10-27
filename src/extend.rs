//! Integer extensions
//!
//! This module contains several traits that dictate behavior for extending
//! an integer past it's range.

// Modules
pub mod default;
pub mod sign;
pub mod zero;

// Exports
pub use default::{Extend, Extended};
pub use sign::{SignExtend, SignExtended};
pub use zero::{ZeroExtend, ZeroExtended};
