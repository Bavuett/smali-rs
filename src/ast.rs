// Abstract Syntax Tree Implementation.

use crate::span::{self, Span};

pub struct SmaliFile<'a> {
    pub class: ClassDeclaration<'a>,
}

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
    // We need a box because, without it, the size of ClassDeclaration would be infinite. Instead, by putting
    // it in the heap by storing it inside a Box.
    pub super_class: Option<ClassDescriptor<'a>>,
}

pub struct MethodDeclaration<'a> {
    pub span: span::Span,
    pub access_modifiers: Vec<Identifier<'a>>,
    pub name: Identifier<'a>,
    pub instructions: Vec<Instruction<'a>>,
}

pub enum Instruction<'a> {
    AddInt {
        destination: Register,
        operand1: Register,
        operand2: Register,
        span: Span,
    },
    ReturnObject {
        register: Register,
        span: Span,
    },
    ReturnVoid {
        span: Span,
    },
    Raw {
        span: span::Span,
        text: &'a str,
    },
}

pub struct Register {
    pub span: span::Span,
    pub number: usize,
}
