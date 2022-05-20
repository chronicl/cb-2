use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    // #[token("/")]
    // Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[regex("[0-9]+")]
    ConstInt,
    #[regex(r#"((([0-9]+\.[0-9]*)|(\.[0-9]+))([eE]([-+])?[0-9]+)?)|([0-9]+[eE]([-+])?[0-9]+)"#)]
    ConstFloat,
    #[regex("true|false")]
    ConstBoolean,
    #[regex(r#""[^\n"]*""#)]
    ConstString,
    #[regex(r#"[a-zA-Z]+([0-9]+|[a-zA-Z]+)*"#)]
    Id,
    #[error]
    #[regex(r#"(//[^\n]*)|(/\*[^(\*/)]*\*/)"#, logos::skip)]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    Error,
}

#[test]
fn tests_c1token() {
    let src = std::fs::read_to_string("tests/resources/demorgan.c1").unwrap();
    let mut lexer = C1Token::lexer(&src);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}
