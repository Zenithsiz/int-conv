//! Default extension

// Imports
use crate::{SignExtend, ZeroExtend};

/// Generic extension
///
/// This type performs either a zero extend or
/// a sign extend, depending if the type is signed.
pub trait Extend<T>: Sized {
	/// Extends this type
	fn extend(self) -> T;
}

/// Extending to the same type simply returns it
impl<T> Extend<T> for T {
	#[inline]
	fn extend(self) -> Self {
		self
	}
}

/// Macro to help implement [`Extend`]
///
/// Note: Regardless if `GAT`s are available, a `impl Extend<&'b U> for &'a T` isn't
///       possible, as it would require memory available for `U` at `T`, which we don't
///       know from just receiving a reference to `T`.
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

			// TODO: Replace with generic version once specialization is stable
			impl<'a> Extend<$U> for &'a $T
			where
				$T: Extend<$U>
			{
				#[inline]
				fn extend(self) -> $U {
					<$T as Extend<$U>>::extend(*self)
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

// Check that all `Extend` impls exist
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
	// Imports
	use super::*;
	use crate::{SignExtended, ZeroExtended};

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
