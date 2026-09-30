// Abstract Syntax Tree Implementation.

use crate::span;

pub struct Identifier<'a> {
    pub span: span::Span,
    pub name: &'a str,
}

pub struct ClassDescriptor<'a> {
    pub span: span::Span,
    pub name: &'a str,
}

pub struct ClassDeclaration<'a> {
    pub span: span::Span,
    pub access: Identifier<'a>,
    pub name: ClassDescriptor<'a>,
}

pub struct Register {
    pub span: span::Span,
    pub number: usize,
}
