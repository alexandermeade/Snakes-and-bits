
use std::env;
use std::fs;

mod lexer;
mod token;
mod run;

fn get_path() -> String {
    let args:Vec<String> = env::args().collect();
    let mut path = String::from("");
    for arg in args {
        if !arg.starts_with('-') {
            path = arg;
        }
    }
    return path;
}

fn main() {

    let path = get_path();

    let contents = fs::read_to_string(&path)
        .expect(&format!("\n\n cannot read from file {}\n\n", &path)); 

    let toks = lexer::Lexer::start(&contents);

    let height = lexer::Lexer::height(&contents);
    
    run::Executer::start(toks, contents, height);  
}
