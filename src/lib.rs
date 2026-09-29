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
}
