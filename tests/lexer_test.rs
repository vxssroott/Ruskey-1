use ruskey::lexer::Lexer;
use ruskey::token::TokenType;

#[test]
fn test_next_token_sequence() {
    let input = "let five = 5; let ten = 10; let add = fn(x, y) { x + y; }; let result = add(five, ten);".to_string();
    let mut l = Lexer::new(input);

    let tests: Vec<(TokenType, &str)> = vec![
        (TokenType::Let, "let"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "ten"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "add"),
        (TokenType::Assign, "="),
        (TokenType::Function, "fn"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::Rbrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::Rparen, ")"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    for (i, expected) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(tok.token_type, expected.0, "tests[{}]: token type wrong. got={:?}, want={:?}", i, tok.token_type, expected.0);
        assert_eq!(tok.literal, expected.1, "tests[{}]: literal wrong. got='{}', want='{}'", i, tok.literal, expected.1);
    }
}
