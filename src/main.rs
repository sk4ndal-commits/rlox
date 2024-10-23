mod interpreter;
mod scanner;
mod token;

use crate::interpreter::interpreter::Interpreter;

fn main() {
    
    /*
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 { 
        println!("Please specify a filename");
        return;
    }
    
    println!("Running {:?}", args[1]);
     */
    
    let file_name = "samples/Hello.rlox";
    
    println!("Running, {}!", file_name);
    
    let mut interpreter = Interpreter::new();
    let chars = interpreter.run(&file_name.as_ref());
    
    println!("Tokens: {:?}", chars);
}
