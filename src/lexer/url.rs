use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex(r#"<a [^h]*href[^>]*>[^<]*</\s*a\s*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r#"<a [^h]*[^r][^>]*>"#, logos::skip)]
    #[regex(r#".|\n"#, logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let mut link = lex.slice();
    let href = "href";
    link = &link[link.find(href).unwrap() + href.len()..];
    assert!(&link[0..2] == "=\"");
    link = &link[2..];
    link = &link[..link.find('"').unwrap()];

    let mut text = lex.slice();
    text = &text[text.find('>').unwrap() + 1..text.find("</").unwrap()];

    (LinkUrl(link.to_owned()), LinkText(text.to_owned()))
}

#[test]
fn tests_url_token() {
    let src = std::fs::read_to_string("tests/resources/urls.html").unwrap();
    let mut lexer = URLToken::lexer(&src);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        println!("{:?}", lexer.slice());
    }
}
