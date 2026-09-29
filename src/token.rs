use std::fmt::{Debug, Display, Formatter};

use crate::span::Span;

#[derive(Debug)]
pub enum TokenKind {
    Directive,       // .class, .method, .end
    Identifier,      // add-int, public, names
    Register,        // v0, p1
    ClassDescriptor, // Ljava/lang/Object;
    EndOfFile,       // EOF
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
