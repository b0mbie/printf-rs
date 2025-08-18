//! Utilities for formatting with the `printf` family of functions in the C standard library.

#![no_std]

use ::core::ffi::*;

mod byte_char;
pub use byte_char::*;
pub mod char_printf;
mod precision;
pub use precision::*;

/// Trait for general conversions of types to those that are [`Formattable`].
pub trait IntoFormattable {
	/// Type that is [`Formattable`].
	type Formattable: Formattable;
	/// Converts a value of the implementing type into one that is [`Formattable`].
	fn into_formattable(self) -> Self::Formattable;
}

impl<T: Formattable> IntoFormattable for T {
	type Formattable = Self;
	fn into_formattable(self) -> Self::Formattable {
		self
	}
}

/// Trait for types that can be formatted with the `printf` family of functions in the C standard library.
/// 
/// # Safety
/// [`format_string`](Formattable::format_string) must return a valid C `printf` format string for the
/// [`CType`](Formattable::CType) and [`Precision`](Formattable::Precision).
pub unsafe trait Formattable {
	/// C `printf` format string.
	type FormatString: AsRef<CStr>;
	/// Returns the appropriate C `printf` format string to format this value,
	/// which is used alongside the [`precision`](Formattable::precision).
	fn format_string(&self) -> Self::FormatString;

	/// Precision of the formatting.
	type Precision: IntoPrecision;
	/// Returns the appropriate precision value for the previously-returned [`format_string`](Formattable::format_string).
	fn precision(&self) -> Self::Precision;

	/// C type that is to be formatted with the previously-returned
	/// [`format_string`](Formattable::format_string) and [`precision`](Formattable::precision).
	type CType;
	/// Converts a value of the implementing type into one that can be used with `printf`.
	fn into_c_type(self) -> Self::CType;
}

unsafe impl Formattable for &CStr {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%s"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = *const c_char;
	fn into_c_type(self) -> Self::CType {
		self.as_ptr()
	}
}

impl<'a> IntoFormattable for &'a str {
	type Formattable = &'a [u8];
	fn into_formattable(self) -> Self::Formattable {
		self.as_bytes()
	}
}

unsafe impl Formattable for &[u8] {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%.*s"
	}

	type Precision = c_int;
	fn precision(&self) -> Self::Precision {
		self.len().min(c_int::MAX as usize) as _
	}

	type CType = *const c_char;
	fn into_c_type(self) -> Self::CType {
		self.as_ptr() as *const _
	}
}

unsafe impl Formattable for &[c_char] {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%.*s"
	}

	type Precision = c_int;
	fn precision(&self) -> Self::Precision {
		self.len().min(c_int::MAX as usize) as _
	}

	type CType = *const c_char;
	fn into_c_type(self) -> Self::CType {
		self.as_ptr()
	}
}

unsafe impl<T> Formattable for *const T {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%p"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = *const c_void;
	fn into_c_type(self) -> Self::CType {
		self as *const _
	}
}

unsafe impl<T> Formattable for *mut T {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%p"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = *mut c_void;
	fn into_c_type(self) -> Self::CType {
		self as *mut _
	}
}

unsafe impl Formattable for f32 {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%g"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = c_double;
	fn into_c_type(self) -> Self::CType {
		self as _
	}
}
unsafe impl Formattable for f64 {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%g"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = c_double;
	fn into_c_type(self) -> Self::CType {
		self as _
	}
}

unsafe impl Formattable for c_schar {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%hhd"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_uchar {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%hhu"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}

unsafe impl Formattable for c_short {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%hd"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_ushort {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%hu"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_int {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%d"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_uint {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%u"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_longlong {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%lld"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for c_ulonglong {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%llu"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for isize {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%td"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl Formattable for usize {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%zu"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
