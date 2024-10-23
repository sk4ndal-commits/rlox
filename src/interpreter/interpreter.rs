use std::fs;
use std::path::Path;
use crate::scanner::scanner::Scanner;
use crate::token::token::Token;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    
    pub fn run(&mut self, file_path: &Path) -> Result<Vec<Token>, String> {
        let source_code = fs::read_to_string(file_path).map_err(|err| {
            format!("Error reading file {}: {}", file_path.to_string_lossy(), err)
        })?;
        let tokens = self.scan(&source_code).map_err(|err| {
            format!("Error scanning tokens: {}.", err.to_string())
        })?;
        Ok(tokens)
    }
    
    fn scan<'a>(&mut self, source_code: &'a str) -> Result<Vec<Token>, &'a str> {
        let mut scanner = Scanner::new(source_code.to_string());
        scanner.scan_tokens()
    }
}