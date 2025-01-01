//! The intermediate representation of the `mlang`.
//!

/// Defines a `identifier` of one node or one attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident(pub String);

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
/// Defines an data.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    /// The identifier name of this element.
    pub ident: Option<Ident>,
    /// the non-inherited properties
    pub fields: Vec<Field>,
}

/// Defines an enum data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// The identifier name of this element.
    pub ident: Ident,
    /// the non-inherited properties
    pub fields: Vec<(Ident, Data)>,
}

/// The property of one node or attr.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// filed name,
    pub ident: Option<Ident>,
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
    Bool,
    String,
    Byte,
    Ubyte,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    /// Data reference by `ident`.
    Data(Ident),
    /// enum data reference by `ident`.
    Enum(Ident),
    /// This type is a listof `type`.
    ListOf(Box<Type>),
}

/// Defines `mlang`'s opcode.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Comment(String),
    Element(Box<Element>),
    Leaf(Box<Leaf>),
    Attr(Box<Attr>),
    Data(Box<Data>),
    Enum(Box<Enum>),
}
