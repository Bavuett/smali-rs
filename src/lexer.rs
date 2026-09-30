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

use crate::{
    error::LexError,
    span::Span,
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    // Look at the current character without going forward.
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(character) = self.peek() {
            if !predicate(character) {
                break;
            }

            self.bump();
        }
    }

    // Look at the current character by `peek()`ing it. Then, bump up the position by looking
    // at the size in bytes of the utf_8 character. Some may be 1 bytes long, some 2, etc.
    // By using `?` on `peek()`, if it returns None() (reached EOF) we can quit the function
    // early without incrementing `pos` and returing None() without explicitly checking EOF and
    // implicitly returning said value.
    fn bump(&mut self) -> Option<char> {
        let character: char = self.peek()?;
        self.pos += character.len_utf8();

        Some(character)
    }

    fn skip_whitespace(&mut self) -> () {
        self.eat_while(|character| character.is_whitespace())
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let start: usize = self.pos;

        let token_result: Result<Token, LexError> = match self.bump() {
            Some('.') => {
                // The '.' has alreasy been consumed by the match arm above.
                // Now we need to eat the rest of the directive.
                self.eat_while(|character| character == '-' || character.is_ascii_alphabetic());

                let end: usize = self.pos;

                let token = Token {
                    kind: TokenKind::Directive,
                    span: Span { start, end },
                };

                Ok(token)
            }
            Some('L') => {
                self.eat_while(|character| {
                    character == '-' || character == '_' || character == '/'
                });

                let kind = if self.peek() == Some(';') {
                    // Consume the token before assigning the ClassDescriptor as its kind: otherwise
                    // the next read will have ';' again and we will be stuck in a loop that gets us
                    // a LexError because it doesn't have any matching arm in this function.
                    self.bump();
                    TokenKind::ClassDescriptor
                } else {
                    TokenKind::Identifier
                };

                let end = self.pos;

                let token = Token {
                    kind,
                    span: Span { start, end },
                };

                Ok(token)
            }
            Some(character)
                if (character == 'v' || character == 'p') && {
                    if let Some(next_char) = self.peek() {
                        next_char.is_ascii_digit()
                    } else {
                        false
                    }
                } =>
            {
                let end = self.pos;

                let token = Token {
                    kind: TokenKind::Register,
                    span: Span { start, end },
                };

                Ok(token)
            }
            // Check is the character an ASCII alphabetic character. If so, it's an identifier.
            // Consume the rest of the identifier and return it as a token.
            // The first character must be an alphabetic character, followed by alphanumeric characters or underscores.
            Some(character) if character.is_ascii_alphabetic() => {
                self.eat_while(|character| {
                    character.is_ascii_alphanumeric()
                        || character == '-'
                        || character == '_'
                        || character == '/'
                });

                let end: usize = self.pos;

                let token = Token {
                    kind: TokenKind::Identifier,
                    span: Span { start, end },
                };

                Ok(token)
            }
            Some(character) if character == '#' => {
                self.eat_while(|character| character != '\n');
                let end: usize = self.pos;
                let token = Token {
                    kind: TokenKind::Comment,
                    span: Span { start, end },
                };

                Ok(token)
            }
            Some(character) if character.is_ascii_digit() => {
                self.eat_while(|character| character.is_ascii_digit());
                let end: usize = self.pos;
                let token = Token {
                    kind: TokenKind::Number,
                    span: Span { start, end },
                };

                Ok(token)
            }
            None => {
                let end: usize = self.pos;

                let token = Token {
                    kind: TokenKind::EndOfFile,
                    span: Span { start, end },
                };

                Ok(token)
            }
            _ => {
                let end: usize = self.pos;

                Err(LexError {
                    message: "Character not handled".to_string(),
                    span: Span { start, end },
                })
            }
        };

        // No need to use the match expression because we already have a `Result` that
        // we can use with `?` to propagate errors.
        let token = token_result?;

        Ok(token)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.next_token()?;
            tokens.push(token);

            if token.kind == TokenKind::EndOfFile {
                break;
            }
        }

        Ok(tokens)
    }
}
