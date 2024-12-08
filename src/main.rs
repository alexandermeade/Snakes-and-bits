
use std::fs;
use colored::Colorize;

mod lexer;
mod token;
mod run;

fn fancy_file(contents:&String) {
    let mut index:usize = 0;
    let mut c = contents.chars().nth(index).unwrap();
    while c != '\0' {
        match c { 
            'S' => print!("{}", "S".green().bold()),
            // stack related commands
            '>' => print!("{}", ">".yellow().bold()),
            '<' => print!("{}", "<".yellow().bold()),
            ',' => print!("{}", ",".yellow().bold()),
            //alphanumeric commands
            'P' => print!("{}", "P".bold()),
            'C' => print!("{}", "P".bold()),
            '#' => print!("{}", "#".yellow().bold()),
            '~' => print!("{}", "~".blue().bold()),
            '@' => print!("{}", "@".red().bold()),
            ' ' => print!("."),
            '\t' => print!("...."),
            '\n' => print!("\\n\n"),
            '"' => {
                let start = index;
                index += 1;
                for i in index..contents.len() {
                    let ch = contents.chars().nth(i).unwrap();
                    if ch == '"' {
                        index = i;
                        break; 
                    }
                }
                index += 1;

                print!("{}", contents[start as usize..index as usize].to_string().green().bold());
                index -=1;

            }
            //computational
            '?' => print!("{}", "?".blue().bold()),
            '\0' => return,
            _ => print!("{}", c)
        }  
        index += 1;
        match contents.chars().nth(index) {
            Some(ch) => c=ch,
            None => return
        }
    }
        

}

fn main() {
    let path = "./src/testing.sab";

    let contents = fs::read_to_string(path)
        .expect("unable to read file");
    
    let toks:Vec<Vec<token::Token>> = lexer::Lexer::start(&contents);

    run::Executer::start(toks, contents); 
    

    //println!("{}", contents);
}
