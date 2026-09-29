use std::fmt::{Display, Formatter};

use crate::span::Span;

#[derive(Debug, PartialEq)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

// NOTE: how `Display`, `fmt`, `Formatter` and `write!` work together.
//
// `Display` is a trait: it tells Rust how to turn a value into user-facing
// text. Once a type implements it, the value can be used with `{}` in any
// formatting macro: `println!("{}", err)`, `format!("{}", err)`,
// `panic!("{}", err)`, and `err.to_string()` also comes for free.
// (Its sibling `Debug` does the same for `{:?}`, meant for developers.)
//
// The trait has a single method, `fmt`. We never call it ourselves: when
// Rust meets `{}` with a `LexError`, it calls `fmt` for us, passing in:
// - `&self`: the error to print;
// - `formatter: &mut Formatter`: the *destination* of the text. It may end up on the
//   terminal, inside a `String`, in a file... `fmt` does not know or care:
//   it just writes into `formatter`. It is `&mut` because writing modifies it.
//
// `write!(formatter, "...", args...)` is the tool to write into `f`. It uses the
// exact same template syntax as `println!` and `format!`: every `{}` or
// `{:?}` is replaced, in order, by the arguments that follow. The only
// difference is where the text goes:
// - `println!("...")`   -> prints it on the terminal;
// - `format!("...")`    -> builds and returns a new `String`;
// - `write!(f, "...")`  -> writes it into its first argument, `f`.
//
// Inside the template:
// - `{}`   formats the argument with its `Display` (here `self.message`, a
//          `String`, prints its text);
// - `{:?}` formats it with its `Debug` (here `self.span` prints as
//          `Span { start: 0, end: 1 }`).
//
// Writing can fail (e.g. the terminal is closed), so `write!` returns a
// `std::fmt::Result`: `Ok(())` on success, `Err(fmt::Error)` otherwise.
// That is exactly what `fmt` must return, so `write!` is the last line and
// has NO semicolon: its value becomes the return value of `fmt`.
//
// Example: `LexError { message: "Unexpected character: #", span: 0..1 }`
// printed with `{}` gives:
//     LexError: Unexpected character: # at Span { start: 0, end: 1 }
//
// WARNING: never write `write!(f, "{}", self)` inside `Display::fmt`: `{}`
// on `self` calls this same `fmt` again, forever (stack overflow).
impl Display for LexError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "LexError: {} at {:?}", self.message, self.span)
    }
}
