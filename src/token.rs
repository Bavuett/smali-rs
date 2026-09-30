use std::fmt::Debug;

use crate::span::Span;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Directive,       // .class, .method, .end
    Identifier,      // add-int, public, names
    Register,        // v0, p1
    ClassDescriptor, // Ljava/lang/Object;
    Comma,           // ,
    Comment,         // # comments
    Number,          // 123, 45.67
    EndOfFile,       // EOF
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
