mod dynamic;

pub use dynamic::*;

use crate::Reflect;

use std::any::{Any, TypeId};

/// A container for compile-time info related to general value types, including primitives.
///
/// This typically represents a type which cannot be broken down any further. This is often
/// due to technical reasons (or by definition), but it can also be a purposeful choice.
///
/// For example, [`i32`] cannot be broken down any further, so it is represented by a [`ValueInfo`].
/// And while [`String`] itself is a struct, it's fields are private, so we don't really treat
/// it _as_ a struct. It therefore makes more sense to represent it as a [`ValueInfo`].
#[derive(Debug, Clone)]
pub struct ValueInfo {
    type_name: &'static str,
    type_id: TypeId,
    #[cfg(feature = "documentation")]
    docs: Option<&'static str>,
}

impl ValueInfo {
    pub fn new<T: Reflect + ?Sized>() -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
            #[cfg(feature = "documentation")]
            docs: None,
        }
    }

    /// Sets the docstring for this value.
    #[cfg(feature = "documentation")]
    pub fn with_docs(self, doc: Option<&'static str>) -> Self {
        Self { docs: doc, ..self }
    }

    /// The [type name] of the value.
    ///
    /// [type name]: std::any::type_name
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The [`TypeId`] of the value.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches the value type.
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// The docstring of this dynamic value, if any.
    #[cfg(feature = "documentation")]
    pub fn docs(&self) -> Option<&'static str> {
        self.docs
    }
}
