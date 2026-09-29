use crate::span::Span;

pub enum TokenKind {
    Directive,       // .class, .method, .end
    Identifier,      // add-int, public, names
    Register,        // v0, p1
    ClassDescriptor, // Ljava/lang/Object;
    EndOfFile,       // EOF
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
