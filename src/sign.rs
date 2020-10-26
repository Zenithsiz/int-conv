//! Types with signed and unsigned variants
//!
//! This modules focuses on describing types that have both an unsigned and signed variant,
//! such as `i8` / `u8`.

/// Types with signed and unsigned variants
///
/// Note that references don't currently implement this trait due to
/// lack of `GAT`s, which are required to specify that a `&'a u8` may
/// be cast to a `&'a i8` with the same lifetime.
pub trait Signed {
	/// Signed variant of this type
	type Signed;

	/// Unsigned variant of this type
	type Unsigned;

	/// Reinterprets this value as unsigned
	fn as_unsigned(self) -> Self::Unsigned;

	/// Reinterprets this value as signed
	fn as_signed(self) -> Self::Signed;

	/// Returns the absolute value of `self` as unsigned.
	fn abs_unsigned(self) -> Self::Unsigned;

	// TODO: Maybe add a `fn signal() -> Signal` method? Or maybe two `is_positive` / `is_negative` methods.
}

/// All types that are signed
pub trait IsSigned: Signed<Signed = Self> {}
impl<T: Signed<Signed = T>> IsSigned for T {}

/// All types that are unsigned
pub trait IsUnsigned: Signed<Unsigned = Self> {}
impl<T: Signed<Unsigned = T>> IsUnsigned for T {}

/// Macro to help implement [`Signed`]
macro_rules! impl_signed {
	(- $TSigned:ty : + $TUnsigned:ty) => {
		// Make sure `T` has the same size as it's unsigned variant
		::static_assertions::assert_eq_size!($TSigned, $TUnsigned);

		impl Signed for $TSigned {
			type Signed = $TSigned;
			type Unsigned = $TUnsigned;

			#[inline]
			#[allow(clippy::as_conversions)]
			fn as_unsigned(self) -> Self::Unsigned {
				// Casting between integers of the same size is a no-op
				self as $TUnsigned
			}

			#[inline]
			fn as_signed(self) -> Self::Signed {
				self
			}

			#[inline]
			fn abs_unsigned(self) -> Self::Unsigned {
				// Note: Branch is optimized by compiler in release mode.
				if self < 0 {
					// Note: We don't use `-self.as_unsigned()` because it can panic
					(!self.as_unsigned()).wrapping_add(1)
				} else {
					self.as_unsigned()
				}
			}
		}

		impl Signed for $TUnsigned {
			type Signed = $TSigned;
			type Unsigned = $TUnsigned;

			#[inline]
			fn as_unsigned(self) -> Self::Unsigned {
				self
			}

			#[inline]
			#[allow(clippy::as_conversions)]
			fn as_signed(self) -> Self::Signed {
				// Casting between integers of the same size is a no-op
				self as $TSigned
			}

			#[inline]
			fn abs_unsigned(self) -> Self::Unsigned {
				// Note: We're already unsigned
				self
			}
		}
	};
}

impl_signed! { -i8    : +u8   }
impl_signed! { -i16   : +u16  }
impl_signed! { -i32   : +u32  }
impl_signed! { -i64   : +u64  }
impl_signed! { -i128  : +u128 }
impl_signed! { -isize : +usize }

// Check that they all implement `Signed` / `IsSigned` / `IsUnsigned`
static_assertions::assert_impl_all! { i8   : Signed, IsSigned   }
static_assertions::assert_impl_all! { i16  : Signed, IsSigned   }
static_assertions::assert_impl_all! { i32  : Signed, IsSigned   }
static_assertions::assert_impl_all! { i64  : Signed, IsSigned   }
static_assertions::assert_impl_all! { i128 : Signed, IsSigned   }
static_assertions::assert_impl_all! { isize: Signed, IsSigned   }
static_assertions::assert_impl_all! { u8   : Signed, IsUnsigned }
static_assertions::assert_impl_all! { u16  : Signed, IsUnsigned }
static_assertions::assert_impl_all! { u32  : Signed, IsUnsigned }
static_assertions::assert_impl_all! { u64  : Signed, IsUnsigned }
static_assertions::assert_impl_all! { u128 : Signed, IsUnsigned }
static_assertions::assert_impl_all! { usize: Signed, IsUnsigned }

// Check that all associated types are correct
static_assertions::assert_type_eq_all! { <i8    as Signed>::Signed, <u8    as Signed>::Signed, i8    }
static_assertions::assert_type_eq_all! { <i16   as Signed>::Signed, <u16   as Signed>::Signed, i16   }
static_assertions::assert_type_eq_all! { <i32   as Signed>::Signed, <u32   as Signed>::Signed, i32   }
static_assertions::assert_type_eq_all! { <i64   as Signed>::Signed, <u64   as Signed>::Signed, i64   }
static_assertions::assert_type_eq_all! { <i128  as Signed>::Signed, <u128  as Signed>::Signed, i128  }
static_assertions::assert_type_eq_all! { <isize as Signed>::Signed, <usize as Signed>::Signed, isize }
static_assertions::assert_type_eq_all! { <i8    as Signed>::Unsigned, <u8    as Signed>::Unsigned, u8    }
static_assertions::assert_type_eq_all! { <i16   as Signed>::Unsigned, <u16   as Signed>::Unsigned, u16   }
static_assertions::assert_type_eq_all! { <i32   as Signed>::Unsigned, <u32   as Signed>::Unsigned, u32   }
static_assertions::assert_type_eq_all! { <i64   as Signed>::Unsigned, <u64   as Signed>::Unsigned, u64   }
static_assertions::assert_type_eq_all! { <i128  as Signed>::Unsigned, <u128  as Signed>::Unsigned, u128  }
static_assertions::assert_type_eq_all! { <isize as Signed>::Unsigned, <usize as Signed>::Unsigned, usize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn as_unsigned_positive() {
		assert_eq!(u8   ::as_unsigned(1), 1);
		assert_eq!(u16  ::as_unsigned(1), 1);
		assert_eq!(u32  ::as_unsigned(1), 1);
		assert_eq!(u64  ::as_unsigned(1), 1);
		assert_eq!(u128 ::as_unsigned(1), 1);
		assert_eq!(usize::as_unsigned(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn as_unsigned_negative() {
		assert_eq!(i8   ::as_unsigned(-1), u8   ::MAX);
		assert_eq!(i16  ::as_unsigned(-1), u16  ::MAX);
		assert_eq!(i32  ::as_unsigned(-1), u32  ::MAX);
		assert_eq!(i64  ::as_unsigned(-1), u64  ::MAX);
		assert_eq!(i128 ::as_unsigned(-1), u128 ::MAX);
		assert_eq!(isize::as_unsigned(-1), usize::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn as_unsigned_negative_big() {
		assert_eq!(i8   ::as_unsigned(i8   ::MIN), u8   ::MAX / 2 + 1);
		assert_eq!(i16  ::as_unsigned(i16  ::MIN), u16  ::MAX / 2 + 1);
		assert_eq!(i32  ::as_unsigned(i32  ::MIN), u32  ::MAX / 2 + 1);
		assert_eq!(i64  ::as_unsigned(i64  ::MIN), u64  ::MAX / 2 + 1);
		assert_eq!(i128 ::as_unsigned(i128 ::MIN), u128 ::MAX / 2 + 1);
		assert_eq!(isize::as_unsigned(isize::MIN), usize::MAX / 2 + 1);
	}

	#[test]
	#[rustfmt::skip]
	fn as_signed_positive() {
		assert_eq!(u8   ::as_signed(1), 1);
		assert_eq!(u16  ::as_signed(1), 1);
		assert_eq!(u32  ::as_signed(1), 1);
		assert_eq!(u64  ::as_signed(1), 1);
		assert_eq!(u128 ::as_signed(1), 1);
		assert_eq!(usize::as_signed(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn as_signed_negative() {
		assert_eq!(i8   ::as_signed(u8   ::MAX.as_signed()), -1);
		assert_eq!(i16  ::as_signed(u16  ::MAX.as_signed()), -1);
		assert_eq!(i32  ::as_signed(u32  ::MAX.as_signed()), -1);
		assert_eq!(i64  ::as_signed(u64  ::MAX.as_signed()), -1);
		assert_eq!(i128 ::as_signed(u128 ::MAX.as_signed()), -1);
		assert_eq!(isize::as_signed(usize::MAX.as_signed()), -1);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_unsigned() {
		assert_eq!(u8   ::abs_unsigned(1), 1);
		assert_eq!(u16  ::abs_unsigned(1), 1);
		assert_eq!(u32  ::abs_unsigned(1), 1);
		assert_eq!(u64  ::abs_unsigned(1), 1);
		assert_eq!(u128 ::abs_unsigned(1), 1);
		assert_eq!(usize::abs_unsigned(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_unsigned_big() {
		assert_eq!(u8   ::abs_unsigned(u8   ::MAX), u8   ::MAX);
		assert_eq!(u16  ::abs_unsigned(u16  ::MAX), u16  ::MAX);
		assert_eq!(u32  ::abs_unsigned(u32  ::MAX), u32  ::MAX);
		assert_eq!(u64  ::abs_unsigned(u64  ::MAX), u64  ::MAX);
		assert_eq!(u128 ::abs_unsigned(u128 ::MAX), u128 ::MAX);
		assert_eq!(usize::abs_unsigned(usize::MAX), usize::MAX);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_signed_positive() {
		assert_eq!(i8   ::abs_unsigned(1), 1);
		assert_eq!(i16  ::abs_unsigned(1), 1);
		assert_eq!(i32  ::abs_unsigned(1), 1);
		assert_eq!(i64  ::abs_unsigned(1), 1);
		assert_eq!(i128 ::abs_unsigned(1), 1);
		assert_eq!(isize::abs_unsigned(1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_signed_positive_big() {
		assert_eq!(i8   ::abs_unsigned(i8   ::MAX), u8   ::MAX / 2);
		assert_eq!(i16  ::abs_unsigned(i16  ::MAX), u16  ::MAX / 2);
		assert_eq!(i32  ::abs_unsigned(i32  ::MAX), u32  ::MAX / 2);
		assert_eq!(i64  ::abs_unsigned(i64  ::MAX), u64  ::MAX / 2);
		assert_eq!(i128 ::abs_unsigned(i128 ::MAX), u128 ::MAX / 2);
		assert_eq!(isize::abs_unsigned(isize::MAX), usize::MAX / 2);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_signed_negative() {
		assert_eq!(i8   ::abs_unsigned(-1), 1);
		assert_eq!(i16  ::abs_unsigned(-1), 1);
		assert_eq!(i32  ::abs_unsigned(-1), 1);
		assert_eq!(i64  ::abs_unsigned(-1), 1);
		assert_eq!(i128 ::abs_unsigned(-1), 1);
		assert_eq!(isize::abs_unsigned(-1), 1);
	}

	#[test]
	#[rustfmt::skip]
	fn abs_unsigned_signed_negative_big() {
		assert_eq!(i8   ::abs_unsigned(i8   ::MIN), u8   ::MAX / 2 + 1);
		assert_eq!(i16  ::abs_unsigned(i16  ::MIN), u16  ::MAX / 2 + 1);
		assert_eq!(i32  ::abs_unsigned(i32  ::MIN), u32  ::MAX / 2 + 1);
		assert_eq!(i64  ::abs_unsigned(i64  ::MIN), u64  ::MAX / 2 + 1);
		assert_eq!(i128 ::abs_unsigned(i128 ::MIN), u128 ::MAX / 2 + 1);
		assert_eq!(isize::abs_unsigned(isize::MIN), usize::MAX / 2 + 1);
	}
}
