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
    fn use_lexer() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/example.smali");

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

        println!("{:?}", token)
    }
}
