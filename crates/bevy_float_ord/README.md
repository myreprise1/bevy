A wrapper for floats that implements [`Ord`], [`Eq`], and [`Hash`] traits.

This is a work around for the fact that the IEEE 754-2008 standard,
implemented by Rust's [`f32`] type,
doesn't define an ordering for [`NaN`](f32::NAN),
and `NaN` is not considered equal to any other `NaN`.

Wrapping a float with `FloatOrd` breaks conformance with the standard
by sorting `NaN` as less than all other numbers and equal to any other `NaN`.