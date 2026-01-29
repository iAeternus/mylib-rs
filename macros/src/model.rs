use syn::{Expr, Generics, Ident, Type};

pub struct BuilderStruct {
    pub name: Ident,
    pub generics: Generics,
    pub fields: Vec<BuilderField>,
}

pub struct BuilderField {
    pub name: Ident,
    pub ty: Type,
    pub skip: bool,
    pub default: Option<Expr>,
}
