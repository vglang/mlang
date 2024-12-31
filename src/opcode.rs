//! The intermediate representation of the `mlang`.
//!

/// Defines a `identifier` of one node or one attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident(pub String);

// impl<T> From<T> for Ident
// where
//     String: From<T>,
// {
//     fn from(value: T) -> Self {
//         Self(value.into())
//     }
// }

/// Defines a non-leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
    /// allow child nodes.
    pub allow_children: Vec<Ident>,
    /// appliable attrs.
    pub allow_attrs: Vec<Ident>,
}

/// Defines a leaf node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Leaf {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
    /// appliable attrs.
    pub allow_attrs: Vec<Ident>,
}

/// Defines an attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attr {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// filed name,
    pub ident: Ident,
    /// The type of this field.
    pub ty: Type,
    /// Indicate this field is optional.
    pub optional: bool,
    /// Indicate this field is a variable field.
    pub variable: bool,
}

/// Defines a `type` of one field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    String,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    /// Data reference by `ident`.
    Data(Ident),
    /// This type is a listof `type`.
    ListOf(Box<Type>),
}

/// Defines `mlang`'s opcode.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Element(Box<Element>),
    Leaf(Box<Leaf>),
    Attr(Box<Attr>),
}
