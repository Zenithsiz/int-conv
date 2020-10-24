//! Integer truncation
//!
//! This module contains the [`Truncate`] trait used for
//! truncating integers to a smaller integer

// Imports
use core::mem;

/// Truncates this integer to a lower size
pub trait Truncate<T>: Sized {
	/// Performs the truncation
	fn truncate(self) -> T;
}

/// Truncating to the same type simply returns it
impl<T> Truncate<T> for T {
	#[inline]
	fn truncate(self) -> T {
		self
	}
}

/// Macro to help implement `Truncate`
///
/// Note: We don't currently `Truncate<&'b U> for &'a T` due
///       to requiring `GAT`s, but we do implement `Truncate<U> for &'a T`
///       by simply copying the underlying type.
macro_rules! impl_truncate {
	($T:ty => $($U:ty),* $(,)?) => {
		$(
			// Make sure `T` is larger than `U`, so we don't extend it.
			::static_assertions::const_assert!(mem::size_of::<$T>() > mem::size_of::<$U>());

			impl Truncate<$U> for $T {
				#[inline]
				#[allow(clippy::as_conversions)]
				fn truncate(self) -> $U {
					// Casting from a larger to a smaller integer truncates
					self as $U
				}
			}

			impl<'a> Truncate<$U> for &'a $T {
				#[inline]
				fn truncate(self) -> $U {
					<$T as Truncate<$U>>::truncate(*self)
				}
			}
		)*
	};
}

// Unsigned
impl_truncate! { u128 => u64, u32, u16, u8 }
impl_truncate! { u64  =>      u32, u16, u8 }
impl_truncate! { u32  =>           u16, u8 }
impl_truncate! { u16  =>                u8 }

// Signed
impl_truncate! { i128 => i64, i32, i16, i8 }
impl_truncate! { i64  =>      i32, i16, i8 }
impl_truncate! { i32  =>           i16, i8 }
impl_truncate! { i16  =>                i8 }

/// Helper trait for [`Truncate`] to be used with turbofish syntax
pub trait Truncated {
	/// Truncates this type
	#[inline]
	fn truncated<T>(self) -> T
	where
		Self: Truncate<T>,
	{
		self.truncate()
	}
}
impl<T> Truncated for T {}

// Check that all `Truncate` impls exist
static_assertions::assert_impl_all! { i128 : Truncate<i128>, Truncate<i64>, Truncate<i32>, Truncate<i16>, Truncate<i8> }
static_assertions::assert_impl_all! { i64  :                 Truncate<i64>, Truncate<i32>, Truncate<i16>, Truncate<i8> }
static_assertions::assert_impl_all! { i32  :                                Truncate<i32>, Truncate<i16>, Truncate<i8> }
static_assertions::assert_impl_all! { i16  :                                               Truncate<i16>, Truncate<i8> }
static_assertions::assert_impl_all! { i8   :                                                              Truncate<i8> }
static_assertions::assert_impl_all! { u128 : Truncate<u128>, Truncate<u64>, Truncate<u32>, Truncate<u16>, Truncate<u8> }
static_assertions::assert_impl_all! { u64  :                 Truncate<u64>, Truncate<u32>, Truncate<u16>, Truncate<u8> }
static_assertions::assert_impl_all! { u32  :                                Truncate<u32>, Truncate<u16>, Truncate<u8> }
static_assertions::assert_impl_all! { u16  :                                               Truncate<u16>, Truncate<u8> }
static_assertions::assert_impl_all! { u8   :                                                              Truncate<u8> }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn truncate_unsigned() {
		assert_eq!(u128::truncated::<u128>(1), 1);
		assert_eq!(u128::truncated::< u64>(1), 1);
		assert_eq!(u128::truncated::< u32>(1), 1);
		assert_eq!(u128::truncated::< u16>(1), 1);
		assert_eq!(u128::truncated::<  u8>(1), 1);
		assert_eq!( u64::truncated::< u64>(1), 1);
		assert_eq!( u64::truncated::< u32>(1), 1);
		assert_eq!( u64::truncated::< u16>(1), 1);
		assert_eq!( u64::truncated::<  u8>(1), 1);
		assert_eq!( u32::truncated::< u32>(1), 1);
		assert_eq!( u32::truncated::< u16>(1), 1);
		assert_eq!( u32::truncated::<  u8>(1), 1);
		assert_eq!( u16::truncated::< u16>(1), 1);
		assert_eq!( u16::truncated::<  u8>(1), 1);
		assert_eq!(  u8::truncated::<  u8>(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn truncate_signed() {
		assert_eq!(i128::truncated::<i128>(-1), -1);
		assert_eq!(i128::truncated::< i64>(-1), -1);
		assert_eq!(i128::truncated::< i32>(-1), -1);
		assert_eq!(i128::truncated::< i16>(-1), -1);
		assert_eq!(i128::truncated::<  i8>(-1), -1);
		assert_eq!( i64::truncated::< i64>(-1), -1);
		assert_eq!( i64::truncated::< i32>(-1), -1);
		assert_eq!( i64::truncated::< i16>(-1), -1);
		assert_eq!( i64::truncated::<  i8>(-1), -1);
		assert_eq!( i32::truncated::< i32>(-1), -1);
		assert_eq!( i32::truncated::< i16>(-1), -1);
		assert_eq!( i32::truncated::<  i8>(-1), -1);
		assert_eq!( i16::truncated::< i16>(-1), -1);
		assert_eq!( i16::truncated::<  i8>(-1), -1);
		assert_eq!(  i8::truncated::<  i8>(-1), -1);
	}
}
