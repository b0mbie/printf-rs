use ::core::{
	convert::Infallible,
	ffi::c_int,
};

/// Marker for types that specify a precision.
pub trait PrecisionSome {}
impl PrecisionSome for c_int {}
impl PrecisionSome for Infallible {}

/// Marker for types that indicate a lack of a precision specifier.
pub trait PrecisionNone {}
impl PrecisionNone for () {}
impl PrecisionNone for Infallible {}

/// Trait for conversions into C `printf` precision specifiers.
pub trait IntoPrecision {
	/// Type that specifies the precision.
	type Some: PrecisionSome;
	/// Type that specifies the lack of a precision.
	type None: PrecisionNone;
	/// Converts a value of the implementing type into a [`Result`] that could hold a C `printf` precision specifier.
	fn into_precision(self) -> Result<Self::Some, Self::None>;
}
impl IntoPrecision for () {
	type Some = Infallible;
	type None = ();
	fn into_precision(self) -> Result<Self::Some, Self::None> {
		Err(())
	}
}
impl IntoPrecision for c_int {
	type Some = c_int;
	type None = Infallible;
	fn into_precision(self) -> Result<Self::Some, Self::None> {
		Ok(self)
	}
}
impl IntoPrecision for Option<c_int> {
	type Some = c_int;
	type None = ();
	fn into_precision(self) -> Result<<Self as IntoPrecision>::Some, <Self as IntoPrecision>::None> {
		match self {
			Some(precision) => Ok(precision),
			None => Err(()),
		}
	}
}
