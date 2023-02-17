use crate::Reflect;
use std::any::{Any, TypeId};

/// The named field of a reflected struct.
pub type NamedField = Field<&'static str>;

/// The unnamed field of a reflected tuple or tuple struct.
pub type UnnamedField = Field<usize>;

/// A field of a reflected struct, tuple, or tuple struct.
/// 
/// Named fields have a key of `&'static str`, accessed by `name`.
/// Unnamed fields (like in tuples and tuple structs) have a key of `usize`, accessed by `index`.
#[derive(Clone, Debug)]
pub struct Field<Key> {
    key: Key,
    type_name: &'static str,
    type_id: TypeId,
    #[cfg(feature = "documentation")]
    docs: Option<&'static str>,
}

impl<Key> Field<Key> {
    /// Create a new [`StructField`].
    pub fn new<T: Reflect>(key: Key) -> Self {
        Self {
            key,
            type_name: std::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
            #[cfg(feature = "documentation")]
            docs: None,
        }
    }
    
    /// The [type name] of the field.
    ///
    /// [type name]: std::any::type_name
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The [`TypeId`] of the field.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches the field type.
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// Sets the docstring for this field.
    #[cfg(feature = "documentation")]
    pub fn with_docs(self, docs: Option<&'static str>) -> Self {
        Self { docs, ..self }
    }

    /// The docstring of this field, if any.
    #[cfg(feature = "documentation")]
    pub fn docs(&self) -> Option<&'static str> {
        self.docs
    }
}

impl Field<&'static str> {
    /// Returns the name of the field.
    pub fn name(&self) -> &'static str {
        self.key
    }
}

impl Field<usize> {
    /// Returns the index of the field.
    pub fn index(&self) -> usize {
        self.key
    }
}