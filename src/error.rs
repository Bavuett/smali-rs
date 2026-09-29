use crate::span::Span;

pub struct LexError {
    pub message: String,
    pub span: Span,
}
