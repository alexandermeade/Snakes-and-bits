use crate::token::TokenType;
use crate::token::Token;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use colored::ColoredString;
use std::io::{stdout, Write};

use crossterm::{terminal::{ClearType, Clear}, QueueableCommand, cursor::{MoveTo, Hide}};

pub fn clear_screen() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
} 

pub struct Executer {
    contents: String,
    board:Vec<Vec<Token>>,
    curr_token: Token,
    index_line: usize,
    stop_running: bool,
    outputs: Vec<String>,
    stack: Vec<i32>,
    stack_index: usize,
    eq_flag:bool,
    neq_flag:bool,
    lt_flag:bool,
    gt_flag:bool,
    lteq_flag:bool,
    gteq_flag:bool,
    not_flag:bool
}

impl Executer {

    fn find_start(&mut self) {
        let mut j: usize = 0;
        
        for tokens in &self.board {
            for token in tokens {
                if token.token_type() == TokenType::Start {
                    self.curr_token = token.clone();
                    self.index_line = j;
                    return;
                }
            }
            j+=1;
        }
    }

    pub fn start(board: Vec<Vec<Token>>, contents:String) {
        let mut exec = Executer {
            contents,
            board,
            curr_token: Token::new(TokenType::NA('_'), 0, 0),
            index_line: 0,
            stop_running: false,
            outputs:Vec::new(),
            stack:Vec::new(),
            stack_index: 0,
            eq_flag:false,
            neq_flag:false,
            lt_flag:false,
            gt_flag:false,
            lteq_flag:false,
            gteq_flag:false,
            not_flag:false
        };
        exec.find_start();
        exec.run();
    }

    fn stack_get_next(&self) -> Option<i32> {
        if self.stack_index + 1 >= self.stack.len() {
            return None;
        }
        return Some(self.stack[self.stack_index + 1]);
    }
    fn get_prev(&mut self) {
        
        let toks = self.board[self.index_line].clone();
        let mut index = 0;
        for i in 0..toks.len() {
            if toks[i].colmn() == self.curr_token.colmn() {
                index = i;
                break;
            }
        }

        if index - 1< 0 || index >= toks.len() {
            return;
        }

        self.curr_token = toks[index-1].clone();
    }
   
    fn get_next(&mut self) {
        
        if self.index_line >= self.board.len() {
            self.print_err("add some padding to the file");
            return;
        }

        let toks = self.board[self.index_line].clone();
        let mut index = 0;
        for i in 0..toks.len() {
            if toks[i].colmn() == self.curr_token.colmn() {
                index = i;
                break;
            }
        }

        if index < 0 || index + 1 >= toks.len() {
            return;
        }

        self.curr_token = toks[index+1].clone();
    }
    
    fn get_above(&mut self) {
         if self.index_line - 1 < 0 {
            self.print_err("Error cannot slide down snakes into nothingness");
            return;
        }
        self.index_line -= 1;
        let toks = self.board[self.index_line].clone();
        let mut index = 0;
        let mut found = false;
        for i in 0..toks.len() {
            if toks[i].colmn() == self.curr_token.colmn() && toks[i].line() != self.index_line.try_into().unwrap() {
                index = i;
                break;
            }
        }

        self.curr_token = toks[index].clone();
      
    }

    fn get_below(&mut self) {
        
        if self.index_line + 1 >= self.board.len() {
            self.print_err("Error cannot slide down snakes into nothingness");
            return;
        }
        self.index_line += 1;
        let toks = self.board[self.index_line].clone();
        let mut index = 0;
        let mut found = false;
        for i in 0..toks.len() {
            if toks[i].colmn() == self.curr_token.colmn() && toks[i].line() != self.index_line.try_into().unwrap(){
                index = i;
                break;
            }
        }

        self.curr_token = toks[index].clone();
    }
    
    fn print_err(&mut self, msg:&str) {
        self.stop_running = true;
        println!("{} {}", "!".red().bold(), msg.to_string().red().bold());
    }
    fn print_curr(&self) {
        println!("exec > {:#?}", self.curr_token); 
    }

    fn run(&mut self) {
        while !self.stop_running {
            clear_screen();
            self.fancy_file_pos_2();
            thread::sleep(Duration::from_millis(100)); 
            match self.curr_token.token_type() {
                TokenType::Start => {
                    //self.print_curr();
                    self.get_next();
                },
                TokenType::Str(string) => {
                    self.outputs.push(format!("{} {}", "~".bold(), string.green()));
                    self.get_next();
                },

                TokenType::Input => {
                    let mut line = String::new();
                    println!("{}", "(Input)".black().bold().on_cyan());
                    let b1 = std::io::stdin().read_line(&mut line).unwrap();  
                    self.outputs.push(format!("{}", line));
                    match format!("{}", line).chars().nth(0) {
                        Some(c) => self.stack.push(c as i32),
                        None => self.print_err("no characters inputed")
                    }
                    self.get_next();
                },
                TokenType::NumInput => {
                    let mut line = String::new();
                    println!("{}", "(Input)".black().bold().on_cyan());
                    let b1 = std::io::stdin().read_line(&mut line).unwrap();  
                    self.outputs.push(format!("{}", line));
                    println!("{}", line);
                    self.stack.push(line.trim().parse().expect("invalid input"));
                    self.get_next();
                },
                TokenType::CopyCmd => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Compare with nothing in the stack");
                        return;
                    }
                    self.stack.push(self.stack[self.stack_index]); 
                    self.get_next();
                },
                TokenType::WipeCmd => {
                    self.stack = Vec::new();

                    self.get_next();
                },
                TokenType::PopCmd => {
                    self.stack.pop();

                    self.get_next();
                },
                TokenType::LeftPan => {
                    self.get_prev();
                },

                TokenType::Front => {
                    self.stack_index = 0;
                    self.get_next();
                },
                TokenType::Back => {
                    self.stack_index = if self.stack.len() as i32-1 < 0 {0} else {self.stack.len()-1};
                    self.get_next();
                },
                TokenType::PrintCell => {
                    self.outputs.push(self.stack[self.stack_index].to_string());
                    self.get_next();
                },
                TokenType::RightShift => {
                    self.stack_index = if self.stack_index + 1 >= self.stack.len() {0} else {self.stack_index + 1};

                    self.get_next();
                },
                TokenType::LeftShift => {
                    self.stack_index = if self.stack_index - 1 < 0 {self.stack.len() -1} else {(self.stack_index as i32 - 1).try_into().unwrap()};

                    self.get_next();
                },
                TokenType::Ladder => {
                    //self.print_curr();
                    self.get_above();
                },
                TokenType::LessThan => {
                    if self.lt_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },
                TokenType::LessThanEqualTo => {
                    if self.lteq_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },
                TokenType::GreaterThan => {
                    if self.gt_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },
                TokenType::GreaterThanEqualTo => {
                    if self.gteq_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },
                TokenType::EqualTo => {
                    if self.eq_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },
                TokenType::NotEqualTo => {
                    if self.neq_flag {
                        self.get_above();
                    }else {
                        self.get_below();
                    }
                },

                TokenType::Compare => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Compare with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => {
                            let left = self.stack[self.stack_index];
                            self.eq_flag = left == value;
                            self.neq_flag = left != value;
                            self.lt_flag = left < value;
                            self.gt_flag = left > value;
                            self.lteq_flag = left <= value;
                            self.gteq_flag = left >= value;
                            self.not_flag = !(if left != 0 {true} else {false});

                        },
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }

                    self.get_next();

                }
                TokenType::Inc => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Increment with nothing in the stack");
                    }
                    self.stack[self.stack_index] += 1;
                    self.get_next();

                },
                TokenType::Dec => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Decrement with nothing in the stack");
                    }

                    self.stack[self.stack_index] -= 1;
                    self.get_next();

                },
                TokenType::Sub => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot subtract with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => self.stack.push(self.stack[self.stack_index] - value),
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }
                    self.get_next();
                },
                TokenType::Mod => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Add with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => self.stack.push(self.stack[self.stack_index] % value),
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }
                    self.get_next();

                },
                TokenType::Div => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Add with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => self.stack.push(self.stack[self.stack_index] / value),
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }
                    self.get_next();

                },
                TokenType::Mult => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Add with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => self.stack.push(self.stack[self.stack_index] * value),
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }
                    self.get_next();

                },
                TokenType::Add => {
                    if self.stack_index >= self.stack.len() {
                        self.print_err("You cannot Add with nothing in the stack");
                    }
                    match self.stack_get_next() {
                        Some(value) => self.stack.push(self.stack[self.stack_index] + value),
                        None => self.print_err("You must have a value adjacent to the right of the stack pointed element to preform arthmatic operations")
                    }
                    self.get_next();

                },
                TokenType::Zero => {
                    self.stack.push(0);
                    self.get_next();
                },
                TokenType::Snake => {
                    self.print_curr();
                    self.get_below();
                },
                TokenType::Step => {
                    self.get_next();
                },
                TokenType::Stop => {
                    println!("exec > {}", "process ended successfully".green().bold());
                    return;
                },
                TokenType::EOF => {
                    return;
                },
                _ => {
                    self.print_curr();
                    self.get_next();
                }
            }
        }
    }

    fn populate_spaces(colmn_count:i32) -> String {

        let spaces = String::from(" ").repeat(colmn_count as usize + 1).to_string();
        format!("{}", spaces)
    }

    fn print_term(&self) {
    }

    fn fancy_bool(value:bool) -> ColoredString{
        if value {
            return "true".blue().bold();
        }
        return "false".red().bold();
    }

    fn fancy_file_pos_2(&self) {
        let mut index:usize = 0;
        let mut c = self.contents.chars().nth(index).unwrap();
        let mut line = 1;
        let mut colmn = 0;
        let mut output = String::from("");
        // printing Flags
        println!( "{}",
            &format!("\n== [{}], != [{}], < [{}], > [{}], <= [{}], >= [{}], ! [{}]\n", 
                Self::fancy_bool(self.eq_flag), 
                Self::fancy_bool(self.neq_flag), 
                Self::fancy_bool(self.lt_flag), 
                Self::fancy_bool(self.gt_flag), 
                Self::fancy_bool(self.lteq_flag), 
                Self::fancy_bool(self.gteq_flag), 
                Self::fancy_bool(self.not_flag)
            )
        );

        //printing stack 
        print!("{}", format!("\n{}\n[", "(Stack)".black().bold().on_cyan()));
        for i in 0..self.stack.len() {
            if i == self.stack_index {
                print!("{}", format!(" {},", self.stack[i].to_string().cyan().bold()));
                continue;
            }
            print!("{}", format!(" {},", self.stack[i].to_string()));
        }
  //      output.push_str(&format!("]\n"));

        //printing code body
        while c != '\0' {
            colmn += 1;
            let res = match c { 
                'S' => format!("{}", "S".green().bold()),
                // stack related commands
                '>' => format!("{}", ">".yellow().bold()),
                '<' => format!("{}", "<".yellow().bold()),
                ',' => format!("{}", ",".yellow().bold()),
                //alphanumeric commands
                'P' => format!("{}", "P".bold()),
                'C' => format!("{}", "C".bold()),
                'W' => format!("{}", "W".bold()),
                '#' => format!("{}", "#".yellow().bold()),
                '~' => format!("{}", "~".blue().bold()),
                '@' => format!("{}", "@".red().bold()),
                '+' => format!("{}", "+".yellow().bold()),
                '-' => format!("{}", "-".yellow().bold()),
                'i' => format!("{}", "i".yellow().bold()),
                'd' => format!("{}", "d".yellow().bold()),
                '0' => format!("{}", "0".blue().bold()),
                ',' => format!("{}", ",".green().bold()),
                ' ' => format!(" "),

                '\t' => {
                    colmn += 4;
                    format!("    ")
                },
            
                '\n' => {
                    colmn = 1;
                    line += 1;
                    format!("\n")
                },
                '"' => {
                    let start = index;
                    index += 1;
                    for i in index..self.contents.len() {
                        let ch = self.contents.chars().nth(i).unwrap();
                        if ch == '"' {
                            index = i;
                            break; 
                        }
                    }
                    index += 1;

                    let inner = format!("{}", self.contents[start as usize..index as usize].to_string().green().bold());
                    colmn += 1;
                    index -=1;
                    inner
             
                }
                //computational
                '?' => format!("{}", "?".blue().bold()),
                '\0' => return,
                _ => format!("{}", c)
            };  
            if self.curr_token.line() == line && self.curr_token.colmn() == colmn-1 { 
                print!("{}", format!("{}", res.on_cyan()));
            }else {
                print!("{}", format!("{}", res));
            }
            index += 1;
            match self.contents.chars().nth(index) {
                Some(ch) => c=ch,
                None => break
            };
        }
        //printing output
        println!("{}{}", String::from("Terminal Output").black().on_cyan(), String::from("\n").on_cyan());

        for out in &self.outputs {
            println!("{} {}", String::from("~").green().bold(), out)
        }
    }

    fn fancy_file_pos(&self) {
        let mut index:usize = 0;
        let mut c = self.contents.chars().nth(index).unwrap();
        let mut prev_colmn = self.curr_token.colmn();
        let mut prev_line = self.curr_token.line();
        print!("\n[");
        for i in 0..self.stack.len() {
            if i == self.stack_index {
                print!(" {},", self.stack[i].to_string().cyan().bold());
                continue;
            }
            print!(" {},", self.stack[i].to_string());
        }
        print!("]\n");

        for tokens in &self.board {
            for token in tokens {
                //making space
                let colmn:i32 = if (token.colmn() as i32-prev_colmn as i32) < 0 {0} else {(token.colmn() as i32-prev_colmn as i32)}; 
                let spaces = Self::populate_spaces(colmn.try_into().unwrap());

                let res = match token.token_type() { 
                    TokenType::Start => format!("{}", "S".green().bold()),
                    // stack related commands
                    TokenType::RightShift => format!("{}", ">".yellow().bold()),
                    TokenType::LeftShift => format!("{}", "<".yellow().bold()),
                    TokenType::Input => format!("{}", ",".yellow().bold()),
                    TokenType::Zero => format!("{}", "0".blue().bold()),
                     //alphanumeric commands
                    //TokenType::Pop => format!("{}", "P".bold()),
                    //'C' => format!("{}", "P".bold()),
                    TokenType::Ladder => format!("{}", "#".yellow().bold()),
                    TokenType::Snake => format!("{}", "~".blue().bold()),
                    TokenType::Stop => format!("{}", "@".red().bold()),
                    TokenType::Str(string) => format!("\"{}\"", string.green().bold()),
                    TokenType::NewLine => String::from("\n"),
                    TokenType::Step => String::from('.'),
                    TokenType::Sub => format!("{}", "-".yellow().bold()),
                    TokenType::Add => format!("{}", "+".yellow().bold()),
                    TokenType::Inc => format!("{}", "i".yellow().bold()),
                    TokenType::Dec => format!("{}", "d".yellow().bold()),
                    TokenType::Front => format!("{}", "[".yellow().italic()),
                    TokenType::Back => format!("{}", "]".yellow().italic()),
                    TokenType::PrintCell => format!("{}", "$".yellow().italic()),
                    TokenType::LeftPan => format!("{}", "\\".yellow().italic()),
                    TokenType::EOF => String::from(""),
//                    //computational
//                '?' => format!("{}", "?".blue().bold()),
//                '\0' => return,
                   _ => format!("{}", "[:?]")
                };

/*                match token.token_type() {
                    TokenType::Str(string) => {
                        prev_colmn = token.colmn_end();
                        prev_line = token.line_end();

                    },
                    _ => {
                        prev_colmn = token.colmn();
                        prev_line = token.line();

                    }
                } 
  */
                prev_colmn = token.colmn();
                prev_line = token.line();

                index += 1;
                
                if token.colmn() == self.curr_token.colmn() && token.line() == self.curr_token.line() {
                    print!("{}", format!("{}{}", spaces, res.normal().black().on_cyan()));
                    self.print_term();
                }else {
                    print!("{}", format!("{}{}", spaces, res));
                }

                match self.contents.chars().nth(index) {
                    Some(ch) => c=ch,
                    None => { 
                        return;
                    }
                }

            }
        }

        println!("{}{}", String::from("Terminal Output").black().on_cyan(), String::from("\n").on_cyan());

        for output in &self.outputs {
            println!("{} {}", String::from("~").green().bold(), output)
        }
    }
 
}

