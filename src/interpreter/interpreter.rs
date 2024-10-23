use std::fs;
use std::path::Path;
use crate::scanner::scanner::Scanner;
use crate::token::token::Token;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    
    pub fn run(&mut self, file_path: &Path) -> Vec<Token> {
        let source_code = fs::read_to_string(file_path).unwrap();
        let tokens = self.scan(&source_code).unwrap();
        tokens
    }
    
    fn scan<'a>(&mut self, source_code: &'a str) -> Result<Vec<Token>, &'a str> {
        let mut scanner = Scanner::new(source_code.to_string());
        scanner.scan_tokens()
    }
}