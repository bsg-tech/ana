pub mod constructs;

use crate::toml::data::*;
use constructs::*;
use std::iter::Peekable;

#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub struct TomlParser {
    pub toml: Toml,
}

impl TomlParser {
    pub fn new() -> Self {
        TomlParser { toml: Toml::new() }
    }

    fn parse_string_array<'a, T: Iterator<Item = &'a Token>>(
        &mut self,
        iter: &mut Peekable<T>,
    ) -> Result<Vec<String>, ParseError> {
        let mut str_arr = Vec::new();
        while let Some(token) = iter.next() {
            match token.token_type {
                // If current token is a string, push it to the array.
                TokenType::String => {
                    str_arr.push(token.lexeme.clone());
                }
                // Array end is kind of unique since arrays can end with ,] or ].
                // Having the case present in both the next and the peek deals with this.
                TokenType::ArrayEnd => {
                    break;
                }
                _ => {
                    return Err(ParseError(String::from(
                        "Parse Error: Unexpected token type in string array.",
                    )));
                }
            }

            // Check that the next token is either a comma or a ]
            if let Some(next_token) = iter.peek() {
                match next_token.token_type {
                    TokenType::Comma => {
                        // Commas can be consumed and ignored.
                        iter.next();
                    }
                    TokenType::ArrayEnd => {
                        break;
                    }
                    _ => {
                        // Otherwise the next toekn is something shady, and we must error.
                        return Err(ParseError(String::from(
                            "Parse Error: the only allowed tokens after a string in an array are , and ]",
                        )));
                    }
                }
            }
        }

        Ok(str_arr)
    }

    fn parse_key_val<'a, T: Iterator<Item = &'a Token>>(
        &mut self,
        kv: &mut KeyVal,
        iter: &mut Peekable<T>,
    ) -> Result<(), ParseError> {
        // Make sure the next token is equals. If not, error out.
        if let Some(next_token) = iter.next()
            && next_token.token_type != TokenType::Equals
        {
            return Err(ParseError(String::from(
                "Parse Error: token after key must be equals sign.",
            )));
        }

        if let Some(value_token) = iter.next() {
            match value_token.token_type {
                TokenType::String => {
                    kv.val = Value::String(value_token.lexeme.clone());
                }
                TokenType::ArrayStart => {
                    let string_array = self.parse_string_array(iter)?;
                    kv.val = Value::ArrayOfStrings(string_array);
                }
                _ => {
                    return Err(ParseError(String::from(
                        "Parse Error: token after equals in a key val pair must be either string or array start.",
                    )));
                }
            }
        } else {
            return Err(ParseError(String::from("error")));
        }

        Ok(())
    }

    fn parse_table<'a, T: Iterator<Item = &'a Token>>(
        &mut self,
        table: &mut Table,
        iter: &mut Peekable<T>,
    ) -> Result<(), ParseError> {
        // Consume table keyval by keyval
        while let Some(token) = iter.next() {
            match token.token_type {
                TokenType::Key => {
                    let mut kv = KeyVal::new(token.lexeme.clone(), None);
                    let _ = self.parse_key_val(&mut kv, iter)?;
                    table.key_vals.push(kv);

                    // Check if the next token is not a key. If it is not, we break!
                    if let Some(next_token) = iter.peek()
                        && next_token.token_type != TokenType::Key
                    {
                        break;
                    }
                }
                _ => {
                    return Err(ParseError::from(String::from(
                        "Parse error: token after table header must be a key",
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), ParseError> {
        let mut iter = tokens.iter().peekable();

        while let Some(token) = iter.next() {
            match token.token_type {
                TokenType::TableHeader => {
                    // TODO: clean up the clone?
                    let mut table = Table::new(token.lexeme.clone());
                    let _ = self.parse_table(&mut table, &mut iter)?;
                    self.toml.tables.push(table);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
