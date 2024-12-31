//! Rust s-expr for `mlang`
use crate::opcode::*;

impl<T> From<T> for Ident
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<T> for Element
where
    Ident: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            fields: vec![],
            allow_attrs: vec![],
            allow_children: vec![],
        }
    }
}

impl Element {
    /// update `allow_attrs` field.
    pub fn allow_attrs<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator,
        Ident: From<I::Item>,
    {
        self.allow_attrs = attrs.into_iter().map(|i| i.into()).collect();
        self
    }

    /// update `allow_children` field.
    pub fn allow_children<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator,
        Ident: From<I::Item>,
    {
        self.allow_children = attrs.into_iter().map(|i| i.into()).collect();
        self
    }

    /// update `allow_attrs` field.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        Field: From<I::Item>,
    {
        self.fields = fields.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl<T> From<T> for Leaf
where
    Ident: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            fields: vec![],
            allow_attrs: vec![],
        }
    }
}

impl Leaf {
    /// update `allow_attrs` field.
    pub fn allow_attrs<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator,
        Ident: From<I::Item>,
    {
        self.allow_attrs = attrs.into_iter().map(|i| i.into()).collect();
        self
    }
    /// update `allow_attrs` field.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        Field: From<I::Item>,
    {
        self.fields = fields.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl<T> From<T> for Attr
where
    Ident: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            fields: vec![],
        }
    }
}

impl Attr {
    /// update `allow_attrs` field.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        Field: From<I::Item>,
    {
        self.fields = fields.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl<T> From<T> for Data
where
    Ident: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: Some(value.into()),
            fields: vec![],
        }
    }
}

impl From<Field> for Data {
    fn from(value: Field) -> Self {
        Self {
            ident: None,
            fields: vec![value],
        }
    }
}

impl From<Type> for Data {
    fn from(value: Type) -> Self {
        Self {
            ident: None,
            fields: vec![Field::from(value)],
        }
    }
}

impl Data {
    /// update `allow_attrs` field.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        Field: From<I::Item>,
    {
        self.fields = fields.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl<T> From<T> for Enum
where
    Ident: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            fields: vec![],
        }
    }
}

impl Enum {
    /// update `fields` field.
    pub fn fields<I, N, F>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = (N, F)>,
        Ident: From<N>,
        Data: From<F>,
    {
        self.fields = fields
            .into_iter()
            .map(|(n, f)| (n.into(), f.into()))
            .collect();
        self
    }
}

impl<I, T> From<(I, T)> for Field
where
    Ident: From<I>,
    Type: From<T>,
{
    fn from(value: (I, T)) -> Self {
        Self {
            ident: Some(value.0.into()),
            ty: value.1.into(),
            optional: false,
            variable: false,
        }
    }
}

impl<T> From<T> for Field
where
    Type: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            ident: None,
            ty: value.into(),
            optional: false,
            variable: false,
        }
    }
}

impl Field {
    /// Set `optional` to true
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Set `variable` to true
    pub fn variable(mut self) -> Self {
        self.variable = true;
        self
    }
}

impl From<Element> for Opcode {
    fn from(value: Element) -> Self {
        Self::Element(Box::new(value))
    }
}

impl From<Leaf> for Opcode {
    fn from(value: Leaf) -> Self {
        Self::Leaf(Box::new(value))
    }
}

impl From<Attr> for Opcode {
    fn from(value: Attr) -> Self {
        Self::Attr(Box::new(value))
    }
}

impl From<Data> for Opcode {
    fn from(value: Data) -> Self {
        Self::Data(Box::new(value))
    }
}

impl From<Enum> for Opcode {
    fn from(value: Enum) -> Self {
        Self::Enum(Box::new(value))
    }
}

impl Type {
    /// Create a data type.
    pub fn data<T>(ident: T) -> Type
    where
        Ident: From<T>,
    {
        Type::Data(ident.into())
    }

    /// Create a enum data type.
    pub fn enum_data<T>(ident: T) -> Type
    where
        Ident: From<T>,
    {
        Type::Enum(ident.into())
    }

    /// Convert self into a list of `Type`.
    pub fn into_list(self) -> Type {
        Type::ListOf(Box::new(self))
    }
}
