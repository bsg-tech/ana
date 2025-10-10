use crate::toml::data::Token;
use crate::toml::data::TokenType;
use io::prelude::*;
use std::fs::File;
use std::io;
use std::iter::Peekable;

pub struct TomlScanner {
    file_path: String,
    pub tokens: Vec<Token>,
}

impl TomlScanner {
    pub fn new(file_path: &str) -> Self {
        TomlScanner {
            file_path: String::from(file_path),
            tokens: Vec::new(),
        }
    }

    fn scan_key<T: Iterator<Item = Result<u8, io::Error>>>(
        &mut self,
        key_str: &mut String,
        iter: &mut Peekable<T>,
    ) -> Result<(), io::Error> {
        if let Some(Ok(toml_byte)) = iter.next() {
            let toml_char = toml_byte as char;

            if self.is_char_valid_th_ascii(toml_char) {
                key_str.push(toml_char);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Malformed key in TOML file, first character must be ascii alphanumeric, -, or _.",
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Malformed key in TOML file, key must have one character.",
            ));
        }

        while let Some(Ok(toml_byte)) = iter.next() {
            let toml_char = toml_byte as char;

            // If we run into a period, we must make sure that the character after is correct
            if toml_char == '.' {
                if let Some(Ok(peeked_byte)) = iter.peek()
                    && !self.is_char_valid_th_ascii(*peeked_byte as char)
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Malformed key header in TOML file: keys containing dots must have an ascii alphanumeric after the dot.",
                    ));
                }
            }

            key_str.push(toml_char);

            // If we run into an equals sign, the key is done.
            if let Some(Ok(peeked_byte)) = iter.peek()
                && *peeked_byte as char == '='
            {
                break;
            }
        }

        Ok(())
    }

    fn skip_comment<T: Iterator<Item = Result<u8, io::Error>>>(
        &mut self,
        iter: &mut Peekable<T>,
    ) -> Result<(), io::Error> {
        while let Some(Ok(_)) = iter.next() {
            if let Some(Ok(peeked_byte)) = iter.peek() {
                if *peeked_byte as char == '\n' {
                    // We cound \n are part of the comment for the sake of parsing, so consume one more time.
                    iter.next();
                    break;
                }
            }
        }

        Ok(())
    }

    fn is_char_valid_th_ascii(&self, toml_char: char) -> bool {
        toml_char.is_ascii_alphanumeric() || toml_char == '-' || toml_char == '_'
    }

    fn scan_table_header<T: Iterator<Item = Result<u8, io::Error>>>(
        &mut self,
        th_str: &mut String,
        iter: &mut Peekable<T>,
    ) -> Result<(), io::Error> {
        // First character after the [ must be alphanumeric, -, or _
        if let Some(Ok(toml_byte)) = iter.next() {
            let toml_char = toml_byte as char;

            if self.is_char_valid_th_ascii(toml_char) {
                th_str.push(toml_char);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Malformed table header in TOML file, first character must be ascii alphanumeric, -, or _.",
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Malformed table header in TOML file, there must be a character after [.",
            ));
        }

        while let Some(Ok(toml_byte)) = iter.next() {
            let toml_char = toml_byte as char;

            // If we run into a period, we must make sure that the character after is correct
            if toml_char == '.' {
                if let Some(Ok(peeked_byte)) = iter.peek()
                    && !self.is_char_valid_th_ascii(*peeked_byte as char)
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Malformed table header in TOML file: table headers containing dots must have an ascii alphanumeric after the dot.",
                    ));
                }
            }

            // If we run into the ], we want to make sure that the next character is \n
            if toml_char == ']' {
                if let Some(Ok(peeked_byte)) = iter.peek()
                    && *peeked_byte as char != '\n'
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Malformed table header in TOML file: table headers must have nothing except a \n after the closing ].",
                    ));
                } else {
                    th_str.push(toml_char);
                    break;
                }
            }

            if self.is_char_valid_th_ascii(toml_char) {
                th_str.push(toml_char);
            }
        }

        Ok(())
    }

    fn scan_string<T: Iterator<Item = Result<u8, io::Error>>>(
        &mut self,
        string_str: &mut String,
        iter: &mut Peekable<T>,
    ) -> Result<(), io::Error> {
        while let Some(Ok(toml_byte)) = iter.next() {
            let toml_char = toml_byte as char;

            if toml_char == '\\' || toml_char == '\n' {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Malformed string on TOML file: character escape is not currently supported.",
                ));
            }

            string_str.push(toml_char);

            // If the current character is ", we have reached the end of the string
            if toml_char == '"' {
                break;
            }
        }

        Ok(())
    }

    pub fn scan(&mut self) -> io::Result<()> {
        // first we read the file and print the lines
        let f = File::open(self.file_path.as_str())?;
        let reader = io::BufReader::new(f);
        let mut iter = reader.bytes().peekable();
        let mut new_line = true;

        while let Some(current_byte_or_err) = iter.next() {
            let current_char = current_byte_or_err? as char;

            match current_char {
                // Complex Scans
                '#' => {
                    new_line = false;
                    match self.skip_comment(&mut iter) {
                        Ok(_) => {}
                        Err(error) => return Err(error),
                    }
                }
                _ if new_line && current_char == '[' => {
                    new_line = false;
                    let mut th_str = String::from(current_char);
                    match self.scan_table_header(&mut th_str, &mut iter) {
                        Ok(_) => {
                            self.tokens.push(Token::new(TokenType::TableHeader, th_str));
                        }
                        Err(error) => return Err(error),
                    }
                }
                '"' => {
                    new_line = false;
                    let mut string_str = String::from(current_char);
                    match self.scan_string(&mut string_str, &mut iter) {
                        Ok(_) => {
                            self.tokens.push(Token::new(TokenType::String, string_str));
                        }
                        Err(error) => return Err(error),
                    }
                }
                _ if new_line && current_char.is_alphabetic() => {
                    new_line = false;
                    let mut key_str = String::from(current_char);
                    match self.scan_key(&mut key_str, &mut iter) {
                        Ok(_) => {
                            self.tokens.push(Token::new(TokenType::Key, key_str));
                        }
                        Err(error) => return Err(error),
                    }
                }
                '\n' => {
                    new_line = true;
                }
                '[' => {
                    self.tokens.push(Token::new(
                        TokenType::ArrayStart,
                        String::from(current_char),
                    ));
                }
                ']' => {
                    self.tokens
                        .push(Token::new(TokenType::ArrayEnd, String::from(current_char)));
                }
                ',' => {
                    self.tokens
                        .push(Token::new(TokenType::Comma, String::from(current_char)));
                }
                '=' => {
                    self.tokens
                        .push(Token::new(TokenType::Equals, String::from(current_char)));
                }
                _ => {}
            }
        }

        Ok(())
    }
}
