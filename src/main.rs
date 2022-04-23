mod lexer;
mod parser;

pub use lexer::{Lexer, reach_eof};

fn main() {
    let mut lexer = Lexer::new("sample/sample0.pl0");
    while !reach_eof(&lexer) {
        let sym = lexer.get_sym();
        println!("{:?}", sym);
    }
}
