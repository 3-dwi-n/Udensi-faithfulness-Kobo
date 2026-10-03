use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };

    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        // Start from the beginning and keep scanning until there are no characters left.
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // Use the last token's line for EOF so extra spaces or new lines at the end
        // don't change where EOF is reported.
        let eof_line = self.tokens.last()
            .map(|token| token.line)
            .unwrap_or(1);

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: eof_line,
        });
    }

    fn scan_token(&mut self) {
        // Get the next character and work out what type of token it starts.
        let c = self.advance();

        match c {
            // These tokens are only one character long.
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            ';' => self.add(TokenType::Semicolon),
            '-' => self.add(TokenType::Minus),
            '+' => self.add(TokenType::Plus),
            '*' => self.add(TokenType::Star),

            // These operators can have an '=' after them.
            '!' => {
                if self.matches('=') {
                    self.add(TokenType::BangEqual);
                } else {
                    self.add(TokenType::Bang);
                }
            }

            '=' => {
                if self.matches('=') {
                    self.add(TokenType::EqualEqual);
                } else {
                    self.add(TokenType::Equal);
                }
            }

            '<' => {
                if self.matches('=') {
                    self.add(TokenType::LessEqual);
                } else {
                    self.add(TokenType::Less);
                }
            }

            '>' => {
                if self.matches('=') {
                    self.add(TokenType::GreaterEqual);
                } else {
                    self.add(TokenType::Greater);
                }
            }

            // A slash can either be division or the start of a comment.
            '/' => {
                if self.matches('/') {
                    // Keep moving until the comment reaches the end of the line.
                    while !self.at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash);
                }
            }

            // Spaces and tabs are ignored because they don't make tokens.
            ' ' | '\r' | '\t' => {}

            // New lines don't make a token, but the line number needs to increase.
            '\n' => {
                self.line += 1;
            }

            // A quote means a string has started.
            '"' => self.string(),

            // Numbers and identifiers need to be read beyond the first character.
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            // Anything else is not recognised by the scanner.
            _ => {
                self.error(
                    self.line,
                    "Character is not part of any token.",
                );
            }
        }
    }

    fn string(&mut self) {
        // Save the starting line because a string can continue onto other lines.
        let start_line = self.line;

        // Keep reading until we find the closing quote.
        while !self.at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        // If there was no closing quote, report the error using the starting line.
        if self.at_end() {
            self.error(start_line, "String is never closed.");
            return;
        }

        // Include the closing quote in the string token.
        self.advance();
        self.add(TokenType::Str);
    }

    fn number(&mut self) {
        // Read all the digits that belong to the number.
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // A dot is part of the number only if another digit comes after it.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add(TokenType::Number);
    }

    fn identifier(&mut self) {
        // Keep reading while the characters can still be part of an identifier.
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.src[self.start..self.current]
            .iter()
            .collect();

        // Check whether the word is a keyword or just a normal identifier.
        let kind = keyword(&text).unwrap_or(TokenType::Identifier);

        self.add(kind);
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        // Check the next character without going past the end of the source.
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        // Look at the current character without moving forward.
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        // Look one character further ahead, mainly for checking decimal numbers.
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        // Everything between start and current makes up the token's text.
        let lexeme: String = self.src[self.start..self.current]
            .iter()
            .collect();

        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        // Store the error so scanning can continue instead of stopping immediately.
        self.errors.push(format!(
            "[line {}] Error: {}",
            line, message
        ));
    }
}