use crate::token::literal::Literal;
use crate::token::literal::Literal::NumberLiteral;
use crate::token::token::Token;
use crate::token::tokentype::TokenType;
use std::collections::HashMap;
use std::sync::OnceLock;

static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, &'static str> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        if c.is_whitespace() {
            // we have to do this here because '\n' is considered `whitespace`
            // as well as '\t', ' ', '\r'
            if c == '\n' { 
                self.line += 1;
            }    
            return;
        }

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.handle_token(TokenType::BangEqual, TokenType::Bang),
            '=' => self.handle_token(TokenType::EqualEqual, TokenType::Equal),
            '<' => self.handle_token(TokenType::LessEqual, TokenType::Less),
            '>' => self.handle_token(TokenType::GreaterEqual, TokenType::Greater),
            '/' => 
                if self.next_token_is('/') {
                    while self.current_token() != '\n' && !self.is_at_end() { self.advance(); }
                }
                else {
                    self.add_token(TokenType::Slash);
                }
            ,
            '"' => self.read_string(),
            _ => 
                if c.is_numeric() { 
                    self.read_number();
                }
                else if c.is_alphabetic() { 
                    self.read_identifier();
                }
                else {
                    eprintln!("Unrecognized character: {} ad line {}", c, self.line);
                    self.add_token(TokenType::Undefined)
                }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_wih_literal(token_type, Option::from(Literal::Null));
    }

    fn add_token_wih_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = match literal{
            Some(Literal::StringLiteral(_)) => self.source[self.start+1..self.current-1].to_owned(),
            _ => self.source[self.start..self.current].to_owned()
        };
        let new_token = Token::new(token_type, lexeme.as_str(), literal, self.line);
        self.tokens.push(new_token);
    }

    fn next_token_is(&mut self, token: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != token {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn handle_token(&mut self, token_match: TokenType, other_match: TokenType) {
        if self.next_token_is('=') {
            self.add_token(token_match)
        } else { self.add_token(other_match) }
    }
    
    fn current_token(&self) -> char {
        if self.is_at_end() {
            '\0'
        }
        else { 
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn read_string(&mut self) {
        while self.current_token() != '"' && !self.is_at_end() {
            if self.current_token() == '\n' { self.line += 1; }
            self.advance();
        }
        
        if self.is_at_end() { 
            eprintln!("[ERROR]: Unterminated string on line {:?}.", self.line);
            return;
        }
        
        self.advance();
        
        let string = self.source[self.start+1..self.current-1].to_string();
        
        self.add_token_wih_literal(TokenType::String, Option::from(Literal::StringLiteral(string)));
    }
    
    fn next_token(&mut self) -> char {
        if self.current+1 >= self.source.len() { 
            '\0' 
        }
        else { 
            self.source.chars().nth(self.current+1).unwrap()
        }
    }
    
    fn read_number(&mut self) {
        while self.current_token().is_numeric() { self.advance(); }
        
        if self.current_token() == '.' && self.next_token().is_numeric() { 
            
            self.advance();
            while self.current_token().is_numeric() { self.advance(); }
        }
        
        let number = self.source[self.start..self.current].parse::<f64>().unwrap().to_owned();
        self.add_token_wih_literal(TokenType::Number, Option::from(NumberLiteral(number)))
    }
    
    fn read_identifier(&mut self) {
        while self.current_token().is_alphanumeric() { self.advance(); }
        
        let identifier = self.source[self.start..self.current].to_owned();
        if let Some(token_type) = self.get_keywords().get(identifier.as_str()) { 
            self.add_token(token_type.to_owned());
            return
        }
        
        self.add_token(TokenType::Identifier)
    }
    
    fn initialize_keywords(&self) -> HashMap<&'static str, TokenType> {
        let mut m = HashMap::new();
        m.insert("and",    TokenType::And);
        m.insert("class",  TokenType::Class);
        m.insert("else",   TokenType::Else);
        m.insert("false",  TokenType::False);
        m.insert("for",    TokenType::For);
        m.insert("fun",    TokenType::Fun);
        m.insert("if",     TokenType::If);
        m.insert("nil",    TokenType::Nil);
        m.insert("or",     TokenType::Or);
        m.insert("print",  TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super",  TokenType::Super);
        m.insert("this",   TokenType::This);
        m.insert("true",   TokenType::True);
        m.insert("var",    TokenType::Var);
        m.insert("while",  TokenType::While);
        m
    }

    fn get_keywords(&self) -> &HashMap<&'static str, TokenType> {
        KEYWORDS.get_or_init(|| self.initialize_keywords())
    }
    
}

