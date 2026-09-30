use crate::{
    ast::{ClassDeclaration, ClassDescriptor, Identifier, Register},
    error::{LexError, ParseError},
    lexer::Lexer,
    span::Span,
    token::{Token, TokenKind},
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

        Ok(ClassDeclaration { span, access, name })
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
