mod interpreter;
mod scanner;
mod token;

use crate::interpreter::interpreter::Interpreter;

fn main() {
    
    let file_name = "samples/Hello.rlox";
    
    println!("Running, {}!", file_name);
    
    let mut interpreter = Interpreter::new();
    match interpreter.run(&file_name.as_ref()) {
        Ok(_tokens) => println!("{:#?}", _tokens),
        Err(error) => println!("Error: {}", error),
    }
}
