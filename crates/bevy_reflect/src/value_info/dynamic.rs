use crate::Reflect;

use std::any::{Any, TypeId};

/// A container for compile-time info related to Bevy's _dynamic_ types, including primitives.
///
/// This is functionally the same as [`ValueInfo`], however, semantically it refers to dynamic
/// types such as [`DynamicStruct`], [`DynamicTuple`], [`DynamicList`], etc.
///
/// [`DynamicStruct`]: crate::DynamicStruct
/// [`DynamicTuple`]: crate::DynamicTuple
/// [`DynamicList`]: crate::DynamicList
#[derive(Debug, Clone)]
pub struct DynamicInfo {
    type_name: &'static str,
    type_id: TypeId,
    #[cfg(feature = "documentation")]
    docs: Option<&'static str>,
}

impl DynamicInfo {
    pub fn new<T: Reflect>() -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
            #[cfg(feature = "documentation")]
            docs: None,
        }
    }

    /// Sets the docstring for this dynamic value.
    #[cfg(feature = "documentation")]
    pub fn with_docs(self, docs: Option<&'static str>) -> Self {
        Self { docs, ..self }
    }

    /// The [type name] of the dynamic value.
    ///
    /// [type name]: std::any::type_name
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The [`TypeId`] of the dynamic value.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches the dynamic value type.
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// The docstring of this value, if any.
    #[cfg(feature = "documentation")]
    pub fn docs(&self) -> Option<&'static str> {
        self.docs
    }
}
