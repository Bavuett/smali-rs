pub mod error;
pub mod lexer;
pub mod span;
pub mod token;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    use super::*;

    #[test]
    fn find_directive() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example.smali");

        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();

        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);

        let token: token::Token = match lexer.next_token() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(token.kind, token::TokenKind::Directive);
        assert_eq!(token.span, span::Span { start: 0, end: 6 });
    }

    #[test]
    fn empty_file() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/empty.smali");

        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();

        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);

        let token: token::Token = match lexer.next_token() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(token.kind, token::TokenKind::EndOfFile);
        assert_eq!(token.span, span::Span { start: 2, end: 2 });
    }

    #[test]
    fn super_smali() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/super.smali");

        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();

        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);

        let token: token::Token = match lexer.next_token() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(token.kind, token::TokenKind::Directive);
        assert_eq!(token.span, span::Span { start: 4, end: 10 });
    }

    #[test]
    fn tokenize() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/super.smali");

        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();

        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);
        let tokens: Vec<token::Token> = match lexer.tokenize() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(tokens[0].kind, token::TokenKind::Directive);
        assert_eq!(tokens[0].span, span::Span { start: 4, end: 10 });

        assert_eq!(tokens[1].kind, token::TokenKind::EndOfFile);
        assert_eq!(tokens[1].span, span::Span { start: 12, end: 12 });
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn class() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/class.smali");
        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();
        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);
        let tokens: Vec<token::Token> = match lexer.tokenize() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(tokens[0].kind, token::TokenKind::Directive);
        assert_eq!(tokens[0].span, span::Span { start: 0, end: 6 });

        assert_eq!(tokens[1].kind, token::TokenKind::Identifier);
        assert_eq!(tokens[1].span, span::Span { start: 7, end: 13 });

        assert_eq!(tokens[2].kind, token::TokenKind::EndOfFile);
        assert_eq!(tokens[2].span, span::Span { start: 15, end: 15 });
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn comments() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/comments.smali");
        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();
        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);
        let tokens: Vec<token::Token> = match lexer.tokenize() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(tokens[0].kind, token::TokenKind::Directive);
        assert_eq!(tokens[0].span, span::Span { start: 0, end: 6 });

        assert_eq!(tokens[1].kind, token::TokenKind::Identifier);
        assert_eq!(tokens[1].span, span::Span { start: 7, end: 17 });

        assert_eq!(tokens[2].kind, token::TokenKind::Comment);
        assert_eq!(tokens[2].span, span::Span { start: 18, end: 25 });

        assert_eq!(tokens[3].kind, token::TokenKind::EndOfFile);
        assert_eq!(tokens[3].span, span::Span { start: 26, end: 26 });
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn tokenize_class_descriptor() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/class_descriptor.smali");
        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();
        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);
        let tokens: Vec<token::Token> = match lexer.tokenize() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(tokens[0].kind, token::TokenKind::Directive);
        assert_eq!(tokens[0].span, span::Span { start: 0, end: 6 });

        assert_eq!(tokens[1].kind, token::TokenKind::Identifier); // "public"
        assert_eq!(tokens[1].span, span::Span { start: 7, end: 13 });

        assert_eq!(tokens[2].kind, token::TokenKind::ClassDescriptor); // "Lcom/example/MyClass;"
        assert_eq!(tokens[2].span, span::Span { start: 14, end: 35 });

        assert_eq!(tokens[3].kind, token::TokenKind::EndOfFile);
        assert_eq!(tokens[3].span, span::Span { start: 37, end: 37 });
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn tokenize_register() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/register.smali");
        let smali_file: File = match File::open(path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: BufReader<File> = BufReader::new(smali_file);
        let mut file_as_string: String = String::new();
        _ = match buffer.read_to_string(&mut file_as_string) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(&file_as_string);
        let tokens: Vec<token::Token> = match lexer.tokenize() {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(tokens[0].kind, token::TokenKind::Register); // "v0"
        assert_eq!(tokens[0].span, span::Span { start: 0, end: 2 });

        assert_eq!(tokens[1].kind, token::TokenKind::EndOfFile);
        assert_eq!(tokens[1].span, span::Span { start: 3, end: 3 });
        assert_eq!(tokens.len(), 2);
    }
}
