//! Types with signed and unsigned variants
//!
//! This modules focuses on describing types that have both an unsigned and signed variant,
//! such as `i8` / `u8`.

/// Types with signed and unsigned variants
pub trait Signed {
	/// Signed variant of this type
	type Signed;

	/// Unsigned variant of this type
	type Unsigned;

	/// Reinterprets this type as unsigned
	fn as_unsigned(self) -> Self::Unsigned;

	/// Reinterprets this type as signed
	fn as_signed(self) -> Self::Signed;

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
	fn as_unsigned() {
		assert_eq!(   u8::as_unsigned(1), 1);
		assert_eq!(  u16::as_unsigned(1), 1);
		assert_eq!(  u32::as_unsigned(1), 1);
		assert_eq!(  u64::as_unsigned(1), 1);
		assert_eq!( u128::as_unsigned(1), 1);
		assert_eq!(usize::as_unsigned(1), 1);
		
		assert_eq!(   i8::as_unsigned(-1), u8   ::MAX);
		assert_eq!(  i16::as_unsigned(-1), u16  ::MAX);
		assert_eq!(  i32::as_unsigned(-1), u32  ::MAX);
		assert_eq!(  i64::as_unsigned(-1), u64  ::MAX);
		assert_eq!( i128::as_unsigned(-1), u128 ::MAX);
		assert_eq!(isize::as_unsigned(-1), usize::MAX);
	}

	#[test]
	#[allow(clippy::cast_possible_wrap)] // We want to wrap around in this test
	#[rustfmt::skip]
	fn as_signed() {
		assert_eq!(   u8::as_signed(1), 1);
		assert_eq!(  u16::as_signed(1), 1);
		assert_eq!(  u32::as_signed(1), 1);
		assert_eq!(  u64::as_signed(1), 1);
		assert_eq!( u128::as_signed(1), 1);
		assert_eq!(usize::as_signed(1), 1);
		
		assert_eq!(   i8::as_signed(u8   ::MAX as    i8), -1);
		assert_eq!(  i16::as_signed(u16  ::MAX as   i16), -1);
		assert_eq!(  i32::as_signed(u32  ::MAX as   i32), -1);
		assert_eq!(  i64::as_signed(u64  ::MAX as   i64), -1);
		assert_eq!( i128::as_signed(u128 ::MAX as  i128), -1);
		assert_eq!(isize::as_signed(usize::MAX as isize), -1);
	}
}
