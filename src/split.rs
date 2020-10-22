//! Integer splitting
//!
//! This module provides ways to split bigger integers
//! into smaller integers and then join them back together.

// Imports
use super::{Truncate, ZeroExtend};
use core::{
	mem,
	ops::{Shl, Shr},
};

/// Splits an integer into it's low and high part
pub trait Split: Sized {
	/// Output type for higher part
	type Hi;

	/// Output type for lower part
	type Lo;

	/// Returns the high part of this integer
	fn hi(self) -> Self::Hi;

	/// Returns the low part of this integer
	fn lo(self) -> Self::Lo;

	/// Returns the low and high part of this integer
	fn lo_hi(self) -> (Self::Lo, Self::Hi);
}

/// Joins two integers into a larger one.
pub trait Join: Split {
	/// Joins two parts of an integer
	fn join(lo: <Self as Split>::Lo, hi: <Self as Split>::Lo) -> Self;
}

// Macro to help implement `Split` / `Join`
macro_rules! impl_split_join {
	($T:ty => $Hi:ty : $Lo:ty) => {
		// Make sure that `T` is made up of `Lo` and `Hi`
		::static_assertions::assert_eq_size!($T, ($Lo, $Hi));

		impl Split for $T {
			type Hi = $Hi;
			type Lo = $Lo;

			#[inline]
			fn lo(self) -> Self::Lo {
				<Self as Truncate<Self::Lo>>::truncate(self)
			}

			#[inline]
			fn hi(self) -> Self::Hi {
				<Self as Truncate<Self::Lo>>::truncate(self.shr(8 * mem::size_of::<Self::Lo>()))
			}

			#[inline]
			fn lo_hi(self) -> (Self::Lo, Self::Hi) {
				let lo = self.lo();
				let hi = self.hi();
				(lo, hi)
			}
		}

		impl Join for $T {
			#[inline]
			fn join(lo: <Self as Split>::Lo, hi: <Self as Split>::Lo) -> Self {
				<$Hi as ZeroExtend<$T>>::zero_extend(hi).shl(8 * mem::size_of::<Self::Lo>()) | <$Lo as ZeroExtend<$T>>::zero_extend(lo)
			}
		}
	};
}

// Unsigned
impl_split_join! { u128 => u64 : u64 }
impl_split_join! { u64  => u32 : u32 }
impl_split_join! { u32  => u16 : u16 }
impl_split_join! { u16  => u8  : u8  }

// Signed
// TODO: Confirm these, should they even exist? Should `Lo` be unsigned?
//impl_split_join! { i128 => i64 : i64 }
//impl_split_join! { i64  => i32 : i32 }
//impl_split_join! { i32  => i16 : i16 }
//impl_split_join! { i16  => i8  : i8  }

// Check that they all implement `Split` / `Join`
//static_assertions::assert_impl_all! { i16  : Split, Join }
//static_assertions::assert_impl_all! { i32  : Split, Join }
//static_assertions::assert_impl_all! { i64  : Split, Join }
//static_assertions::assert_impl_all! { i128 : Split, Join }
static_assertions::assert_impl_all! { u16  : Split, Join }
static_assertions::assert_impl_all! { u32  : Split, Join }
static_assertions::assert_impl_all! { u64  : Split, Join }
static_assertions::assert_impl_all! { u128 : Split, Join }

// Check that all associated types are correct
//static_assertions::assert_type_eq_all! { <i16   as Split>::Lo, <i16   as Split>::Hi, i8  }
//static_assertions::assert_type_eq_all! { <i32   as Split>::Lo, <i32   as Split>::Hi, i16 }
//static_assertions::assert_type_eq_all! { <i64   as Split>::Lo, <i64   as Split>::Hi, i32 }
//static_assertions::assert_type_eq_all! { <i128  as Split>::Lo, <i128  as Split>::Hi, i64 }
static_assertions::assert_type_eq_all! { <u16   as Split>::Lo, <u16   as Split>::Hi, u8  }
static_assertions::assert_type_eq_all! { <u32   as Split>::Lo, <u32   as Split>::Hi, u16 }
static_assertions::assert_type_eq_all! { <u64   as Split>::Lo, <u64   as Split>::Hi, u32 }
static_assertions::assert_type_eq_all! { <u128  as Split>::Lo, <u128  as Split>::Hi, u64 }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn split_lo() {
		assert_eq!(u128::lo(u128::MAX), u64::MAX);
		assert_eq!( u64::lo( u64::MAX), u32::MAX);
		assert_eq!( u32::lo( u32::MAX), u16::MAX);
		assert_eq!( u16::lo( u16::MAX),  u8::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn split_lo_half() {
		assert_eq!(u128::lo(u128::from(u64::MAX)), u64::MAX);
		assert_eq!( u64::lo( u64::from(u32::MAX)), u32::MAX);
		assert_eq!( u32::lo( u32::from(u16::MAX)), u16::MAX);
		assert_eq!( u16::lo( u16::from( u8::MAX)),  u8::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn split_hi() {
		assert_eq!(u128::hi(u128::MAX), u64::MAX);
		assert_eq!( u64::hi( u64::MAX), u32::MAX);
		assert_eq!( u32::hi( u32::MAX), u16::MAX);
		assert_eq!( u16::hi( u16::MAX),  u8::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn split_hi_half() {
		assert_eq!(u128::hi(u128::from(u64::MAX)), 0);
		assert_eq!( u64::hi( u64::from(u32::MAX)), 0);
		assert_eq!( u32::hi( u32::from(u16::MAX)), 0);
		assert_eq!( u16::hi( u16::from( u8::MAX)), 0);
	}

	#[test]
	#[rustfmt::skip]
	fn split_lo_hi() {
		assert_eq!(u128::lo_hi(u128::MAX), (u128::lo(u128::MAX), u128::hi(u128::MAX)));
		assert_eq!( u64::lo_hi( u64::MAX), ( u64::lo( u64::MAX),  u64::hi( u64::MAX)));
		assert_eq!( u32::lo_hi( u32::MAX), ( u32::lo( u32::MAX),  u32::hi( u32::MAX)));
		assert_eq!( u16::lo_hi( u16::MAX), ( u16::lo( u16::MAX),  u16::hi( u16::MAX)));
	}

	#[test]
	#[rustfmt::skip]
	fn split_lo_hi_half() {
		assert_eq!(u128::lo_hi(u128::from(u64::MAX)), (u128::lo(u128::from(u64::MAX)), u128::hi(u128::from(u64::MAX))));
		assert_eq!( u64::lo_hi( u64::from(u32::MAX)), ( u64::lo( u64::from(u32::MAX)),  u64::hi( u64::from(u32::MAX))));
		assert_eq!( u32::lo_hi( u32::from(u16::MAX)), ( u32::lo( u32::from(u16::MAX)),  u32::hi( u32::from(u16::MAX))));
		assert_eq!( u16::lo_hi( u16::from( u8::MAX)), ( u16::lo( u16::from( u8::MAX)),  u16::hi( u16::from( u8::MAX))));
	}
}
