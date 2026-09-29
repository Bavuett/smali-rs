use std::fmt::Debug;

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
