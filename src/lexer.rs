// NOTE: why do we need the `'a` lifetime?
//
// `Lexer` does not own the source: it only holds a `&str` reference to a
// string that lives somewhere else (e.g. the file contents read by the
// caller). A struct that holds a reference must declare how long that
// reference is valid, otherwise the compiler cannot guarantee that the
// referenced data still exists while the struct is using it.
//
// By writing `Lexer<'a>` with `src: &'a str` we are saying: "a `Lexer` cannot
// outlive the string it reads from". This way the borrow checker rejects,
// at compile time, code like the following, which would otherwise produce a
// dangling pointer:
//
//     let lexer;
//     {
//         let source = std::fs::read_to_string("example.smali")?;
//         lexer = Lexer { src: &source, pos: 0 };
//     } // `source` is deallocated here...
//     // ...but `lexer` would still use it -> compile error
//
// The alternative would be to use `String` (owning the data), but copying
// the whole file for every lexer is wasteful: with `&'a str` lexing is
// zero-copy, and later on we can also return `&'a str` slices of the
// source (e.g. the text of a token) without allocating anything.

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    // Look at the current character without going forward.
    pub fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    // Look at the current character by `peek()`ing it. Then, bump up the position by looking
    // at the size in bytes of the utf_8 character. Some may be 1 bytes long, some 2, etc.
    // By using `?` on `peek()`, if it returns None() (reached EOF) we can quit the function
    // early without incrementing `pos` and returing None() without explicitly checking EOF and
    // implicitly returning said value.
    pub fn bump(&mut self) -> Option<char> {
        let character: char = self.peek()?;
        self.pos += character.len_utf8();

        Some(character)
    }

    pub fn skip_whitespace(&mut self) -> () {
        while let Some(character) = self.peek() {
            if character.is_whitespace() {
                // We encountered a space, tab, etc. so we need to bump the counter to the next character.
                self.bump();
            } else {
                // This is not a whitespace, so we get out of the function.
                return;
            }
        }
    }
}
