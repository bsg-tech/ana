use std::fmt;

#[derive(PartialEq)]
pub enum TokenType {
    Key,
    String,
    TableHeader,
    ArrayStart,
    ArrayEnd,
    Comma,
    Equals,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Key => write!(f, "key"),
            TokenType::String => write!(f, "string"),
            TokenType::TableHeader => write!(f, "table header"),
            TokenType::ArrayStart => write!(f, "array start"),
            TokenType::ArrayEnd => write!(f, "array end"),
            TokenType::Comma => write!(f, "comma"),
            TokenType::Equals => write!(f, "equals"),
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}
