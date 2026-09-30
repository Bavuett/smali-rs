use crate::{
    ast::{ClassDeclaration, ClassDescriptor, Identifier, Instruction, MethodDeclaration, Register, SmaliFile}, error::{LexError, ParseError}, lexer::Lexer, span::Span, token::{Token, TokenKind},
};

pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Result<Self, LexError> {
        let mut lexer: Lexer<'_> = Lexer::new(src);

        let current: Token = lexer.next_token()?;

        Ok(Self {
            src,
            lexer,
            current,
        })
    }

    fn bump(&mut self) -> Result<Token, LexError> {
        let previous = self.current;
        self.current = self.lexer.next_token()?;
        Ok(previous)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        let token: Token = self.current;

        if token.kind != expected {
            return Err(ParseError::Unexpected {
                expected,
                got: token.kind,
                span: token.span,
            });
        }

        self.bump()?;
        Ok(token)
    }

    fn parse_class(&mut self) -> Result<ClassDeclaration<'a>, ParseError> {
        let directive: Token = self.expect(TokenKind::Directive)?;
        let span: Span = directive.span;

        let access_token: Token = self.expect(TokenKind::Identifier)?;
        let access: Identifier<'_> = Identifier {
            span: access_token.span,
            name: &self.src[access_token.span.start..access_token.span.end],
        };
        let name_token: Token = self.expect(TokenKind::ClassDescriptor)?;
        let name: ClassDescriptor<'_> = ClassDescriptor {
            span: name_token.span,
            name: &self.src[name_token.span.start..name_token.span.end],
        };

        Ok(ClassDeclaration {
            span,
            access,
            name,
            super_class: None,
        })
    }

    fn parse_method(&mut self) -> Result<MethodDeclaration<'a>, ParseError> {
        let directive: Token = self.expect(TokenKind::Directive)?;
        let start_span = directive.span.start;

        let mut access_modifiers: Vec<Identifier<'a>> = Vec::new();

        let valid_modifiers = [
            "public",
            "private",
            "protected",
            "static",
            "final",
            "abstract",
        ];

        // Continue treating each next Token as an access modifier.
        while self.current.kind == TokenKind::Identifier {
            let span: Span = self.current.span;
            let text: &str = &self.src[span.start..span.end];

            if valid_modifiers.contains(&text) {
                // Valid Modifier! We may consume it and add it.
                let token: Token = self.bump()?;
                access_modifiers.push(Identifier {
                    span: token.span,
                    name: text,
                });
            } else {
                // It IS an Identifier, but it is not in `valid_modifiers`. This means we exhausted all access modifiers.
                // For this reason, we get out of the loop.
                break;
            }
        }

        let method_name: Token = self.expect(TokenKind::Identifier)?;
        let method_name_span: Span = method_name.span;

        let mut instructions = Vec::new();
        let mut end_span = method_name_span.end;

        while self.current.kind != TokenKind::EndOfFile {
            if self.current.kind == TokenKind::Directive {
                let text = &self.src[self.current.span.start..self.current.span.end];

                if text == ".end" {
                    // We found the end of the method. We must consume it.
                    self.expect(TokenKind::Directive)?;

                    // Smali ends its methods by using `.end method`: after consuming `.end`, we need to consume `method`.
                    // We save the Token to get its span.
                    let end_word: Token = self.expect(TokenKind::Identifier)?;
                        end_span = end_word.span.end;

                    // We found the end: we may exit the loop.
                    break;
                } else {
                    // It is another directive interal to the method (example: .registers 2).
                    // For now, we may skip it by consuming the token. We may implement parsing later.
                    self.bump()?;
                    continue;
                }
            }

            let instruction: Instruction<'_> = self.parse_insruction()?;
            instructions.push(instruction);

            let method_declaration: MethodDeclaration<'a> = MethodDeclaration {
                name: Identifier {
                    span: method_name_span,
                    name: &self.src[method_name_span.start..method_name_span.end],
                    },
                access_modifiers,
                span: Span { start: start_span, end: end_span},
                instructions,
            };
        }

        let method_declaration: MethodDeclaration<'a> = MethodDeclaration {
            name: Identifier {
                span: method_name_span,
                name: &self.src[method_name_span.start..method_name_span.end],
            },
            access_modifiers,
            span: Span {
                start: start_span,
                end: 0
            }
        }
    }

    fn parse_insruction(&mut self) -> Result<Instruction<'a>, ParseError> {
        let opcode_token: Token = self.expect(TokenKind::Identifier)?;
        let opcode_text: &str = &self.src[opcode_token.span.start..opcode_token.span.end];

        match opcode_text {
            "return-void" => {
                Ok(
                    Instruction::ReturnVoid { span: opcode_token.span }
                )
            },
            "return-object" => {
                // We need to read a register.
                let register = self.parse_register()?;
                let total_span = Span { start: opcode_token.span.start, end: register.span.end };

                Ok(Instruction::ReturnObject {
                    register,
                    span: total_span
                })
            },
            _ => {
                Ok(Instruction::Raw {
                    span: opcode_token.span,
                    text: opcode_text
                })
            }
        }
    }

    fn parse_super(&mut self) -> Result<ClassDescriptor<'a>, ParseError> {
        self.expect(TokenKind::Directive)?;

        let name_token = self.expect(TokenKind::ClassDescriptor)?;
        let name = ClassDescriptor {
            span: name_token.span,
            name: &self.src[name_token.span.start..name_token.span.end],
        };

        Ok(name)
    }

    fn parse_register(&mut self) -> Result<Register, ParseError> {
        let register_token: Token = self.expect(TokenKind::Register)?;
        let span: Span = register_token.span;

        let number: usize = match self.src[span.start + 1..span.end].parse::<usize>() {
            Ok(number) => number,
            Err(_) => {
                return Err(ParseError::Unexpected {
                    expected: TokenKind::Register,
                    got: TokenKind::Register,
                    span,
                });
            }
        };

        Ok(Register { span, number })
    }

    pub fn parse(&mut self) -> Result<SmaliFile<'a>, ParseError> {
        let mut smali_file: Option<SmaliFile<'a>>;
        let mut class: Option<ClassDeclaration> = None;

        while self.current.kind != TokenKind::EndOfFile {
            // Let's look at the token's text to dinguistish the various directives.
            let text = &self.src[self.current.span.start..self.current.span.end];

            match text {
                ".class" => {
                    class = Some(self.parse_class()?);
                }
                ".super" => {
                    let super_descriptor: ClassDescriptor = self.parse_super()?;

                    if let Some(class_descriptor) = &mut class {
                        class_descriptor.super_class = Some(super_descriptor);
                    }
                }
                ".method" => {}
                _ => {
                    return Err(ParseError::Unexpected {
                        expected: TokenKind::Directive,
                        got: self.current.kind,
                        span: self.current.span,
                    });
                }
            }
        }

        let class = match class {
            Some(class) => class,
            None => {
                return Err(ParseError::Unexpected {
                    expected: TokenKind::Directive,
                    got: self.current.kind,
                    span: self.current.span,
                });
            }
        };

        Ok(SmaliFile { class })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn parse_class() {
        // .class public Lcom/example/MyClass;
        let src = ".class public Lcom/example/MyClass;";
        let mut parser = Parser::new(src).expect("Failed to create parser");
        let class = parser.parse_class().expect("Failed to parse class");

        // span of the directive ".class"
        assert_eq!(class.span, Span { start: 0, end: 6 });

        // access modifier "public"
        assert_eq!(class.access.span, Span { start: 7, end: 13 });
        assert_eq!(class.access.name, "public");

        // class descriptor "Lcom/example/MyClass;"
        assert_eq!(class.name.span, Span { start: 14, end: 35 });
        assert_eq!(class.name.name, "Lcom/example/MyClass;");
    }

    #[test]
    fn parse_register() {
        // v0
        let src = "v0";
        let mut parser = Parser::new(src).expect("Failed to create parser");
        let register = parser.parse_register().expect("Failed to parse register");

        assert_eq!(register.span, Span { start: 0, end: 2 });
        assert_eq!(register.number, 0);
    }

    #[test]
    fn parse_register_double_digit() {
        // v12
        let src = "v12";
        let mut parser = Parser::new(src).expect("Failed to create parser");
        let register = parser.parse_register().expect("Failed to parse register");

        assert_eq!(register.span, Span { start: 0, end: 3 });
        assert_eq!(register.number, 12);
    }
}
