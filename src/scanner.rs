//! Contains a `Scanner` which parses `Tokens` from a `String` of Lox source.
use crate::value::Value;

#[derive(Debug)]
pub struct Scanner {
    pub line: usize,
    src: Vec<char>,
    curr: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            line: 1,
            src: src.chars().collect(),
            curr: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();
        loop {
            vec.push(self.scan_token());
            if vec.last().unwrap().token_type == TokenType::EOF {
                break;
            }
        }
        vec
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();
        if Self::is_alpha(*c) {
            return self.identifier();
        }
        if Self::is_digit(*c) {
            return self.number();
        }

        match c {
            // Single-character tokens
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            // Double or single-character tokens
            '!' => {
                let tok_type = if self.tok_match('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.make_token(tok_type)
            }
            '=' => {
                let tok_type = if self.tok_match('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.make_token(tok_type)
            }
            '<' => {
                let tok_type = if self.tok_match('=') { TokenType::LessEqual } else { TokenType::Less };
                self.make_token(tok_type)
            }
            '>' => {
                let tok_type = if self.tok_match('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.make_token(tok_type)
            }
            '"' => {
                self.string()
            }

            _ => self.make_token(TokenType::Error("Unexpected character".to_string()))
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }

            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    // Check for comment
                    if self.peek_next() == &'/' {
                        // Advance till the end of the line
                        loop {
                            if self.is_at_end() { break; }
                            if self.advance() == &'\n' { break; }
                        }
                    }
                }
                _ => return,
            };
        }
    }

    fn tok_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.nth(self.curr) != &expected {
            return false;
        }
        self.curr += 1;
        true
    }

    fn last(&self) -> &char {
        self.nth(self.curr - 1)
    }

    fn peek(&self) -> &char {
        self.nth(self.curr)
    }

    fn peek_next(&self) -> &char {
        if self.curr + 1 >= self.src.len() {
            return &'\0';
        }

        self.nth(self.curr + 1)
    }

    fn advance(&mut self) -> &char {
        self.curr += 1;
        self.nth(self.curr - 1)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
        }
    }
    fn is_at_end(&self) -> bool {
        self.curr == self.src.len()
    }

    fn nth(&self, n: usize) -> &char {
        self.src.iter().nth(n).expect("Index out of bounds")
    }

    fn string(&mut self) -> Token {
        let mut chars = Vec::new();

        while !self.is_at_end() && self.peek() != &'"' {
            if self.peek() == &'\n' {
                self.line += 1;
            }
            chars.push(*self.advance());
        }

        if self.is_at_end() {
            return self.make_token(TokenType::Error("Unterminated string".to_string()));
        }

        // The closing quote
        self.advance();
        self.make_token(TokenType::String(chars.iter().collect()))
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> Token {
        let mut chars = vec![*self.last()];

        if !self.is_at_end() {
            while Self::is_digit(*self.peek()) {
                chars.push(*self.advance());
            }

            // Look for a fractional part
            if self.peek() == &'.' && Self::is_digit(*self.peek_next()) {
                // Consume the '.'
                chars.push(*self.advance());
                while Self::is_digit(*self.peek()) {
                    chars.push(*self.advance());
                }
            }
        }

        // Coerce the vec of chars to a number value
        let s: String = chars.iter().collect();
        self.make_token(TokenType::Number(s.parse().expect("Expected Value from number")))
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn identifier(&mut self) -> Token {
        let mut chars = vec![*self.last()];

        while !self.is_at_end() && (Self::is_alpha(*self.peek()) || Self::is_digit(*self.peek())) {
            chars.push(*self.advance());
        }

        // We could optimize keyword disambiguation with a trie
        let s: String = chars.iter().collect();
        match s.as_str() {
            "and" => self.make_token(TokenType::And),
            "class" => self.make_token(TokenType::Class),
            "else" => self.make_token(TokenType::Else),
            "false" => self.make_token(TokenType::False),
            "for" => self.make_token(TokenType::For),
            "fun" => self.make_token(TokenType::Fun),
            "if" => self.make_token(TokenType::If),
            "nil" => self.make_token(TokenType::Nil),
            "or" => self.make_token(TokenType::Or),
            "print" => self.make_token(TokenType::Print),
            "return" => self.make_token(TokenType::Return),
            "super" => self.make_token(TokenType::Super),
            "this" => self.make_token(TokenType::This),
            "true" => self.make_token(TokenType::True),
            "var" => self.make_token(TokenType::Var),
            "while" => self.make_token(TokenType::While),
            _ => self.make_token(TokenType::Identifier(s.to_string()))
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(Value),
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special tokens
    Error(String),
    EOF,
}
