//! Sign extension

// Imports
use crate::Signed;
use core::mem;

/// Sign extends
///
/// This trait serves to extend integers with their
/// sign signal.
pub trait SignExtend<T>: Sized {
	/// Sign extends this type
	fn sign_extend(self) -> T;
}

/// Sign extending to the same type simply returns it
impl<T> SignExtend<T> for T {
	#[inline]
	fn sign_extend(self) -> Self {
		self
	}
}

/// Macro to help implement [`SignExtend`]
///
/// Note: Regardless if `GAT`s are available, a `impl SignExtend<&'b U> for &'a T` isn't
///       possible, as it would require memory available for `U` at `T`, which we don't
///       know from just receiving a reference to `T`.
macro_rules! impl_sign_extend {
	($T:ty => $( $U:ty ),+ $(,)?) => {
		$(
			// Make sure `U` is bigger or equal to `T` so we don't truncate the integer
			// Note: It is guaranteed that signed and unsigned variants have the same size
			::static_assertions::const_assert!(mem::size_of::<$U>() >= mem::size_of::<$T>());

			impl SignExtend<$U> for $T
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

			// TODO: Replace with generic version once specialization is stable
			impl<'a> SignExtend<$U> for &'a $T
			where
				$T: SignExtend<$U>
			{
				#[inline]
				fn sign_extend(self) -> $U {
					<$T as SignExtend<$U>>::sign_extend(*self)
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

/// Helper trait for [`SignExtend`] to be used with turbofish syntax
pub trait SignExtended {
	/// Sign extends this type
	#[inline]
	fn sign_extended<T>(self) -> T
	where
		Self: SignExtend<T>,
	{
		self.sign_extend()
	}
}
impl<T> SignExtended for T {}

// Check that all `SignExtend` / `Extend` impls exist
static_assertions::assert_impl_all! { i8   : SignExtend<i8>, SignExtend<i16>, SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i16  :                 SignExtend<i16>, SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i32  :                                  SignExtend<i32>, SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i64  :                                                   SignExtend<i64>, SignExtend<i128> }
static_assertions::assert_impl_all! { i128 :                                                                    SignExtend<i128> }

#[cfg(test)]
mod tests {
	// Imports
	use super::*;

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
}
