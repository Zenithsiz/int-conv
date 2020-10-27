//! Zero extension

// Imports
use crate::Signed;
use core::mem;

/// Zero extend
///
/// This trait serves to extend integers with `0`s,
/// including signed ones.
pub trait ZeroExtend<T>: Sized {
	/// Zero extends this type
	fn zero_extend(self) -> T;
}

/// Zero extending to the same type simply returns it
impl<T> ZeroExtend<T> for T {
	#[inline]
	fn zero_extend(self) -> Self {
		self
	}
}

/// Macro to help implement [`ZeroExtend`]
///
/// Note: Regardless if `GAT`s are available, a `impl ZeroExtend<&'b U> for &'a T` isn't
///       possible, as it would require memory available for `U` at `T`, which we don't
///       know from just receiving a reference to `T`.
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

			// TODO: Replace with generic version once specialization is stable
			impl<'a> ZeroExtend<$U> for &'a $T
			where
				$T: ZeroExtend<$U>
			{
				#[inline]
				fn zero_extend(self) -> $U {
					<$T as ZeroExtend<$U>>::zero_extend(*self)
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

// Check that all `ZeroExtend` impls exist
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

#[cfg(test)]
mod tests {
	// Imports
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
}
