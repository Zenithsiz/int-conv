//! Integer extensions
//!
//! This module contains several traits that dictate behavior for extending
//! an integer past it's range.

// Imports
use crate::{IsSigned, Signed};
use core::mem;

/// Zero extend
///
/// This trait serves to extend integers with `0`s,
/// including signed ones.
pub trait ZeroExtend<T>: Sized {
	/// Zero extends this type
	fn zero_extend(self) -> T;
}

/// Sign extends
///
/// This trait serves to extend integers with their
/// sign signal.
pub trait SignExtend<T: IsSigned>: Sized + IsSigned {
	/// Sign extends this type
	fn sign_extend(self) -> T;
}

/// Generic extension
///
/// This type performs either a zero extend or
/// a sign extend, depending if the type is signed.
pub trait Extend<T>: Sized {
	/// Extends this type
	fn extend(self) -> T;
}

/// Zero extending to the same type simply returns it
impl<T> ZeroExtend<T> for T {
	#[inline]
	fn zero_extend(self) -> Self {
		self
	}
}

/// Sign extending to the same type simply returns it
impl<T: IsSigned> SignExtend<T> for T {
	#[inline]
	fn sign_extend(self) -> Self {
		self
	}
}

/// Extending to the same type simply returns it
impl<T> Extend<T> for T {
	#[inline]
	fn extend(self) -> Self {
		self
	}
}

/// Macro to help implement [`ZeroExtend`]
macro_rules! impl_zero_extend {
	($T:ty => $( $U:ty ),+ $(,)?) => {
		$(
			// Make sure `U` is bigger or equal to `T` so we don't truncate the integer
			// Note: It is guaranteed that signed and unsigned variants have the same size
			::static_assertions::const_assert!(mem::size_of::<$U>() >= mem::size_of::<$T>());

			impl ZeroExtend<$U> for $T
			{
				#[inline]
				#[allow(clippy::as_conversions)]
				fn zero_extend(self) -> $U {
					// Casting between signedness is a no-op.
					// Casting from a smaller to larger unsigned integer will zero-extend.
					self
						as <$T as Signed>::Unsigned
						as <$U as Signed>::Unsigned
						as $U
				}
			}
		)+
	};
}

// Unsigned
impl_zero_extend! { u8   => u16, u32, u64, u128 }
impl_zero_extend! { u16  =>      u32, u64, u128 }
impl_zero_extend! { u32  =>           u64, u128 }
impl_zero_extend! { u64  =>                u128 }

// Signed
impl_zero_extend! { i8   => i16, i32, i64, i128 }
impl_zero_extend! { i16  =>      i32, i64, i128 }
impl_zero_extend! { i32  =>           i64, i128 }
impl_zero_extend! { i64  =>                i128 }

/// Macro to help implement [`SignExtend`]
macro_rules! impl_sign_extend {
	($T:ty => $( $U:ty ),+ $(,)?) => {
		$(
			// Make sure `U` is bigger or equal to `T` so we don't truncate the integer
			// Note: It is guaranteed that signed and unsigned variants have the same size
			::static_assertions::const_assert!(mem::size_of::<$U>() >= mem::size_of::<$T>());

			impl SignExtend<$U> for $T
			where
				$U: IsSigned
			{
				#[inline]
				#[allow(clippy::as_conversions)]
				fn sign_extend(self) -> $U {
					// Casting between signedness is a no-op.
					// Casting from a smaller to larger signed integer will sign-extend.
					self
						as <$T as Signed>::Signed
						as <$U as Signed>::Signed
						as $U
				}
			}
		)+
	};
}

// Signed
impl_sign_extend! { i8   => i16, i32, i64, i128 }
impl_sign_extend! { i16  =>      i32, i64, i128 }
impl_sign_extend! { i32  =>           i64, i128 }
impl_sign_extend! { i64  =>                i128 }

/// Macro to help implement [`ZeroExtend`]
macro_rules! impl_extend {
	($T:ty => $( $U:ty ),+ $(,)? => $method:ident) => {
		$(
			impl Extend<$U> for $T
			{
				#[inline]
				fn extend(self) -> $U {
					self.$method()
				}
			}
		)+
	};
}

// Unsigned
impl_extend! { u8   => u16, u32, u64, u128 => zero_extend }
impl_extend! { u16  =>      u32, u64, u128 => zero_extend }
impl_extend! { u32  =>           u64, u128 => zero_extend }
impl_extend! { u64  =>                u128 => zero_extend }

// Signed
impl_extend! { i8   => i16, i32, i64, i128 => sign_extend }
impl_extend! { i16  =>      i32, i64, i128 => sign_extend }
impl_extend! { i32  =>           i64, i128 => sign_extend }
impl_extend! { i64  =>                i128 => sign_extend }

/// Helper trait for [`ZeroExtend`] to be used with turbofish syntax
pub trait ZeroExtended: Sized {
	/// Zero extends this type
	#[inline]
	fn zero_extended<T>(self) -> T
	where
		Self: ZeroExtend<T>,
	{
		self.zero_extend()
	}
}
impl<T> ZeroExtended for T {}

/// Helper trait for [`SignExtend`] to be used with turbofish syntax
pub trait SignExtended: IsSigned {
	/// Sign extends this type
	#[inline]
	fn sign_extended<T: IsSigned>(self) -> T
	where
		Self: SignExtend<T>,
	{
		self.sign_extend()
	}
}
impl<T: IsSigned> SignExtended for T {}

/// Helper trait for [`Extend`] to be used with turbofish syntax
pub trait Extended {
	/// Extends this type
	#[inline]
	fn extended<T>(self) -> T
	where
		Self: Extend<T>,
	{
		self.extend()
	}
}
impl<T> Extended for T {}

// Check that all `ZeroExtend` / `SignExtend` / `Extend` impls exist
static_assertions::assert_impl_all! { i8   : ZeroExtend<i8>, ZeroExtend<i16>, ZeroExtend<i32>, ZeroExtend<i64>, ZeroExtend<i128> }
static_assertions::assert_impl_all! { i16  :                 ZeroExtend<i16>, ZeroExtend<i32>, ZeroExtend<i64>, ZeroExtend<i128> }
static_assertions::assert_impl_all! { i32  :                                  ZeroExtend<i32>, ZeroExtend<i64>, ZeroExtend<i128> }
static_assertions::assert_impl_all! { i64  :                                                   ZeroExtend<i64>, ZeroExtend<i128> }
static_assertions::assert_impl_all! { i128 :                                                                    ZeroExtend<i128> }
static_assertions::assert_impl_all! { u8   : ZeroExtend<u8>, ZeroExtend<u16>, ZeroExtend<u32>, ZeroExtend<u64>, ZeroExtend<u128> }
static_assertions::assert_impl_all! { u16  :                 ZeroExtend<u16>, ZeroExtend<u32>, ZeroExtend<u64>, ZeroExtend<u128> }
static_assertions::assert_impl_all! { u32  :                                  ZeroExtend<u32>, ZeroExtend<u64>, ZeroExtend<u128> }
static_assertions::assert_impl_all! { u64  :                                                   ZeroExtend<u64>, ZeroExtend<u128> }
static_assertions::assert_impl_all! { u128 :                                                                    ZeroExtend<u128> }
static_assertions::assert_impl_all! { i8   : SignExtend<i8>, SignExtend<i16>, SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i16  :                 SignExtend<i16>, SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i32  :                                  SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i64  :                                                   SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i128 :                                                                    SignExtend<i128> }
static_assertions::assert_impl_all! { i8   :     Extend<i8>,     Extend<i16>,     Extend<i32>,     Extend<i64>,     Extend<i128> }
static_assertions::assert_impl_all! { i16  :                     Extend<i16>,     Extend<i32>,     Extend<i64>,     Extend<i128> }
static_assertions::assert_impl_all! { i32  :                                      Extend<i32>,     Extend<i64>,     Extend<i128> }
static_assertions::assert_impl_all! { i64  :                                                       Extend<i64>,     Extend<i128> }
static_assertions::assert_impl_all! { i128 :                                                                        Extend<i128> }
static_assertions::assert_impl_all! { u8   :     Extend<u8>,     Extend<u16>,     Extend<u32>,     Extend<u64>,     Extend<u128> }
static_assertions::assert_impl_all! { u16  :                     Extend<u16>,     Extend<u32>,     Extend<u64>,     Extend<u128> }
static_assertions::assert_impl_all! { u32  :                                      Extend<u32>,     Extend<u64>,     Extend<u128> }
static_assertions::assert_impl_all! { u64  :                                                       Extend<u64>,     Extend<u128> }
static_assertions::assert_impl_all! { u128 :                                                                        Extend<u128> }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn zero_extend_unsigned() {
		assert_eq!(  u8::zero_extended::<  u8>(1), 1);
		assert_eq!(  u8::zero_extended::< u16>(1), 1);
		assert_eq!(  u8::zero_extended::< u32>(1), 1);
		assert_eq!(  u8::zero_extended::< u64>(1), 1);
		assert_eq!(  u8::zero_extended::<u128>(1), 1);
		assert_eq!( u16::zero_extended::< u16>(1), 1);
		assert_eq!( u16::zero_extended::< u32>(1), 1);
		assert_eq!( u16::zero_extended::< u64>(1), 1);
		assert_eq!( u16::zero_extended::<u128>(1), 1);
		assert_eq!( u32::zero_extended::< u32>(1), 1);
		assert_eq!( u32::zero_extended::< u64>(1), 1);
		assert_eq!( u32::zero_extended::<u128>(1), 1);
		assert_eq!( u64::zero_extended::< u64>(1), 1);
		assert_eq!( u64::zero_extended::<u128>(1), 1);
		assert_eq!(u128::zero_extended::<u128>(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn zero_extend_unsigned_big() {
		
		assert_eq!( u8::zero_extended::< u16>( u8::MAX),  u16::from( u8::MAX));
		assert_eq!( u8::zero_extended::< u32>( u8::MAX),  u32::from( u8::MAX));
		assert_eq!( u8::zero_extended::< u64>( u8::MAX),  u64::from( u8::MAX));
		assert_eq!( u8::zero_extended::<u128>( u8::MAX), u128::from( u8::MAX));
		assert_eq!(u16::zero_extended::< u32>(u16::MAX),  u32::from(u16::MAX));
		assert_eq!(u16::zero_extended::< u64>(u16::MAX),  u64::from(u16::MAX));
		assert_eq!(u16::zero_extended::<u128>(u16::MAX), u128::from(u16::MAX));
		assert_eq!(u32::zero_extended::< u64>(u32::MAX),  u64::from(u32::MAX));
		assert_eq!(u32::zero_extended::<u128>(u32::MAX), u128::from(u32::MAX));
		assert_eq!(u64::zero_extended::<u128>(u64::MAX), u128::from(u64::MAX));
		
		assert_eq!(  u8::zero_extended::<  u8>(  u8::MAX),   u8::MAX);
		assert_eq!( u16::zero_extended::< u16>( u16::MAX),  u16::MAX);
		assert_eq!( u32::zero_extended::< u32>( u32::MAX),  u32::MAX);
		assert_eq!( u64::zero_extended::< u64>( u64::MAX),  u64::MAX);
		assert_eq!(u128::zero_extended::<u128>(u128::MAX), u128::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn zero_extend_signed_positive() {
		assert_eq!(  i8::zero_extended::<  i8>(1), 1);
		assert_eq!(  i8::zero_extended::< i16>(1), 1);
		assert_eq!(  i8::zero_extended::< i32>(1), 1);
		assert_eq!(  i8::zero_extended::< i64>(1), 1);
		assert_eq!(  i8::zero_extended::<i128>(1), 1);
		assert_eq!( i16::zero_extended::< i16>(1), 1);
		assert_eq!( i16::zero_extended::< i32>(1), 1);
		assert_eq!( i16::zero_extended::< i64>(1), 1);
		assert_eq!( i16::zero_extended::<i128>(1), 1);
		assert_eq!( i32::zero_extended::< i32>(1), 1);
		assert_eq!( i32::zero_extended::< i64>(1), 1);
		assert_eq!( i32::zero_extended::<i128>(1), 1);
		assert_eq!( i64::zero_extended::< i64>(1), 1);
		assert_eq!( i64::zero_extended::<i128>(1), 1);
		assert_eq!(i128::zero_extended::<i128>(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn zero_extend_signed_negative() {
		assert_eq!( i8::zero_extended::< i16>(-1),  i16::from(u8  ::MAX));
		assert_eq!( i8::zero_extended::< i32>(-1),  i32::from(u8  ::MAX));
		assert_eq!( i8::zero_extended::< i64>(-1),  i64::from(u8  ::MAX));
		assert_eq!( i8::zero_extended::<i128>(-1), i128::from(u8  ::MAX));
		assert_eq!(i16::zero_extended::< i32>(-1),  i32::from(u16 ::MAX));
		assert_eq!(i16::zero_extended::< i64>(-1),  i64::from(u16 ::MAX));
		assert_eq!(i16::zero_extended::<i128>(-1), i128::from(u16 ::MAX));
		assert_eq!(i32::zero_extended::< i64>(-1),  i64::from(u32 ::MAX));
		assert_eq!(i32::zero_extended::<i128>(-1), i128::from(u32 ::MAX));
		assert_eq!(i64::zero_extended::<i128>(-1), i128::from(u64 ::MAX));
		
		assert_eq!(  i8::zero_extended::<  i8>(-1), -1);
		assert_eq!( i16::zero_extended::< i16>(-1), -1);
		assert_eq!( i32::zero_extended::< i32>(-1), -1);
		assert_eq!( i64::zero_extended::< i64>(-1), -1);
		assert_eq!(i128::zero_extended::<i128>(-1), -1);
	}

	#[test]
	#[rustfmt::skip]
	fn sign_extend_positive() {
		assert_eq!(i8 ::sign_extended::< i16>(1), 1);
		assert_eq!(i8 ::sign_extended::< i32>(1), 1);
		assert_eq!(i8 ::sign_extended::< i64>(1), 1);
		assert_eq!(i8 ::sign_extended::<i128>(1), 1);
		assert_eq!(i16::sign_extended::< i32>(1), 1);
		assert_eq!(i16::sign_extended::< i64>(1), 1);
		assert_eq!(i16::sign_extended::<i128>(1), 1);
		assert_eq!(i32::sign_extended::< i64>(1), 1);
		assert_eq!(i32::sign_extended::<i128>(1), 1);
		assert_eq!(i64::sign_extended::<i128>(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn sign_extend_negative() {
		assert_eq!( i8::sign_extended::< i16>(-1), -1);
		assert_eq!( i8::sign_extended::< i32>(-1), -1);
		assert_eq!( i8::sign_extended::< i64>(-1), -1);
		assert_eq!( i8::sign_extended::<i128>(-1), -1);
		assert_eq!(i16::sign_extended::< i32>(-1), -1);
		assert_eq!(i16::sign_extended::< i64>(-1), -1);
		assert_eq!(i16::sign_extended::<i128>(-1), -1);
		assert_eq!(i32::sign_extended::< i64>(-1), -1);
		assert_eq!(i32::sign_extended::<i128>(-1), -1);
		assert_eq!(i64::sign_extended::<i128>(-1), -1);
	}

	#[test]
	#[rustfmt::skip]
	fn extend_unsigned() {
		assert_eq!( u8::extended::< u16>(1),  u8::zero_extended::< u16>(1));
		assert_eq!( u8::extended::< u32>(1),  u8::zero_extended::< u32>(1));
		assert_eq!( u8::extended::< u64>(1),  u8::zero_extended::< u64>(1));
		assert_eq!( u8::extended::<u128>(1),  u8::zero_extended::<u128>(1));
		assert_eq!(u16::extended::< u32>(1), u16::zero_extended::< u32>(1));
		assert_eq!(u16::extended::< u64>(1), u16::zero_extended::< u64>(1));
		assert_eq!(u16::extended::<u128>(1), u16::zero_extended::<u128>(1));
		assert_eq!(u32::extended::< u64>(1), u32::zero_extended::< u64>(1));
		assert_eq!(u32::extended::<u128>(1), u32::zero_extended::<u128>(1));
		assert_eq!(u64::extended::<u128>(1), u64::zero_extended::<u128>(1));
	}

	#[test]
	#[rustfmt::skip]
	fn extend_unsigned_big() {
		assert_eq!(  u8::extended::<  u8>(  u8::MAX),   u8::zero_extended::<  u8>(  u8::MAX));
		assert_eq!(  u8::extended::< u16>(  u8::MAX),   u8::zero_extended::< u16>(  u8::MAX));
		assert_eq!(  u8::extended::< u32>(  u8::MAX),   u8::zero_extended::< u32>(  u8::MAX));
		assert_eq!(  u8::extended::< u64>(  u8::MAX),   u8::zero_extended::< u64>(  u8::MAX));
		assert_eq!(  u8::extended::<u128>(  u8::MAX),   u8::zero_extended::<u128>(  u8::MAX));
		assert_eq!( u16::extended::< u16>( u16::MAX),  u16::zero_extended::< u16>( u16::MAX));
		assert_eq!( u16::extended::< u32>( u16::MAX),  u16::zero_extended::< u32>( u16::MAX));
		assert_eq!( u16::extended::< u64>( u16::MAX),  u16::zero_extended::< u64>( u16::MAX));
		assert_eq!( u16::extended::<u128>( u16::MAX),  u16::zero_extended::<u128>( u16::MAX));
		assert_eq!( u32::extended::< u32>( u32::MAX),  u32::zero_extended::< u32>( u32::MAX));
		assert_eq!( u32::extended::< u64>( u32::MAX),  u32::zero_extended::< u64>( u32::MAX));
		assert_eq!( u32::extended::<u128>( u32::MAX),  u32::zero_extended::<u128>( u32::MAX));
		assert_eq!( u64::extended::< u64>( u64::MAX),  u64::zero_extended::< u64>( u64::MAX));
		assert_eq!( u64::extended::<u128>( u64::MAX),  u64::zero_extended::<u128>( u64::MAX));
		assert_eq!(u128::extended::<u128>(u128::MAX), u128::zero_extended::<u128>(u128::MAX));
	}

	#[test]
	#[rustfmt::skip]
	fn extend_signed_positive() {
		assert_eq!( i8::extended::< i16>(1),  i8::sign_extended::< i16>(1));
		assert_eq!( i8::extended::< i32>(1),  i8::sign_extended::< i32>(1));
		assert_eq!( i8::extended::< i64>(1),  i8::sign_extended::< i64>(1));
		assert_eq!( i8::extended::<i128>(1),  i8::sign_extended::<i128>(1));
		assert_eq!(i16::extended::< i32>(1), i16::sign_extended::< i32>(1));
		assert_eq!(i16::extended::< i64>(1), i16::sign_extended::< i64>(1));
		assert_eq!(i16::extended::<i128>(1), i16::sign_extended::<i128>(1));
		assert_eq!(i32::extended::< i64>(1), i32::sign_extended::< i64>(1));
		assert_eq!(i32::extended::<i128>(1), i32::sign_extended::<i128>(1));
		assert_eq!(i64::extended::<i128>(1), i64::sign_extended::<i128>(1));
	}

	#[test]
	#[rustfmt::skip]
	fn extend_signed_negative() {
		assert_eq!( i8::extended::< i16>(-1),  i8::sign_extended::< i16>(-1));
		assert_eq!( i8::extended::< i32>(-1),  i8::sign_extended::< i32>(-1));
		assert_eq!( i8::extended::< i64>(-1),  i8::sign_extended::< i64>(-1));
		assert_eq!( i8::extended::<i128>(-1),  i8::sign_extended::<i128>(-1));
		assert_eq!(i16::extended::< i32>(-1), i16::sign_extended::< i32>(-1));
		assert_eq!(i16::extended::< i64>(-1), i16::sign_extended::< i64>(-1));
		assert_eq!(i16::extended::<i128>(-1), i16::sign_extended::<i128>(-1));
		assert_eq!(i32::extended::< i64>(-1), i32::sign_extended::< i64>(-1));
		assert_eq!(i32::extended::<i128>(-1), i32::sign_extended::<i128>(-1));
		assert_eq!(i64::extended::<i128>(-1), i64::sign_extended::<i128>(-1));
	}
}
