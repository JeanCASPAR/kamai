use crate::parser::lexer::{Lexer, Token, TokenElem};

#[test]
fn simple_tokens() {
    let mut lexer = Lexer::new(" ab_cd_e ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(
            TokenElem::Identifier("ab_cd_e".to_string()),
            1..8,
            "ab_cd_e".to_string(),
        )],
    );
    let mut lexer = Lexer::new(" 15 ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(TokenElem::Int(15), 1..3, "15".to_string())],
    );
    let mut lexer = Lexer::new(" = ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(TokenElem::Equal, 1..2, "=".to_string())],
    );
    let mut lexer = Lexer::new(" ; ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(TokenElem::Semicolon, 1..2, ";".to_string())],
    );
    let mut lexer = Lexer::new(" (5) ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(
            TokenElem::ParenthesisPair(vec![Token::new(TokenElem::Int(5), 2..3, "5".to_string())]),
            1..3,
            "(5)".to_string(),
        )],
    );
    let mut lexer = Lexer::new(" { 5; 15 } ".to_string());
    assert_eq!(
        lexer.tokenize().unwrap(),
        vec![Token::new(
            TokenElem::BracketPair(vec![
                Token::new(TokenElem::Int(5), 3..4, "5".to_string()),
                Token::new(TokenElem::Semicolon, 4..5, ";".to_string()),
                Token::new(TokenElem::Int(15), 6..8, "15".to_string())
            ]),
            1..9,
            "{ 5; 15 }".to_string(),
        )],
    )
}
