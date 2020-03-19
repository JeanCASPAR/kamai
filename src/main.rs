mod parser;

use parser::{lexer::Lexer, parser::Parser};

// TODO: Add bracket expression parsing and `;` operator and then typechecking
fn main() {
    let mut lexer = Lexer::new(
        "
            add x y = x + y;
        "
        .to_string(),
    );
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
    let tokens = tokens.unwrap();
    let mut parser = Parser::new(tokens);
    println!("{:#?}", parser.parse())
}
