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
