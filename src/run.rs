use crate::token::TokenType;
use crate::token::Token;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use colored::ColoredString;
use std::io::{stdout, Write};
use std::env;
use crossterm::event;
use crate::run::event::Event;
use crossterm::cursor;
use crossterm::terminal;
use crossterm::{
    execute,
    ExecutableCommand,
};
use std::process;
use std::io;
use rand::Rng;
use ctrlc;
//pub fn clear_screen() {
/*    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
*/


fn clear_screen() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap(); 
    io::stdout().flush().unwrap();
}



pub struct Executer {
    contents: String,
    board:Vec<Vec<Token>>,
    curr_token: Token,
    index_line: usize,
    stop_running: bool,
    outputs: Vec<ColoredString>,
    stack: Vec<i32>,
    stack_index: usize,
    eq_flag:bool,
    neq_flag:bool,
    lt_flag:bool,
    gt_flag:bool,
    lteq_flag:bool,
    gteq_flag:bool,
    not_flag:bool,
    tick_count:i32,
    output_height:u32,
    show_stack:bool,
    show_visualizer:bool,
    show_output:bool,
    show_flags:bool,
    steps: bool,
    plain: bool,
    handle_resize: bool
}

impl Executer {
    
    fn eval_args(&mut self) {
        let args:Vec<String> = env::args().collect();
        for arg in &args {
            match arg as &str {
                "-nvisual" => self.show_visualizer = false,
                "-nstack" => self.show_stack = false,
                "-noutput" => self.show_output = false,
                "-nflags" => self.show_flags = false,
                "-nstep" => self.steps = false,
                "-plain" => {
                    self.plain = true;
                    self.steps = false;
                    self.handle_resize = false
                },
                _ => {}
            }
        } 
    }

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

    pub fn start(board: Vec<Vec<Token>>, contents:String, output_height:u32) {

        ctrlc::set_handler(move || {
            clear_screen();
            process::exit(0);
        }).expect("error setting ctrl c handler");

        let mut exec = Executer {
            contents,
            board,
            curr_token: Token::new(TokenType::NA('_'), 0, 0, 0),
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
            not_flag:false,
            tick_count: 0,
            output_height,
            show_stack: true,
            show_visualizer: true,
            show_output: true,
            show_flags: true,
            steps: true,
            plain: false,
            handle_resize: true
        };
        exec.eval_args();
        exec.find_start();
        clear_screen();
        exec.run();
        
        if exec.plain {
            let _ = exec.outputs.into_iter().map(|s| println!("{}", s));
        }

    }
    
    fn get_rand(bound:i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..bound)
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
        //let mut found = false;
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
        //let mut found = false;
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
        //println!("exec > {:#?}", self.curr_token); 
    }
    fn handle_output(&mut self, output:ColoredString) {
        if self.plain {
            println!("{}", output);
            return;
        }
        self.outputs.push(output);
        
    }
    fn run(&mut self) {

        while !self.stop_running {
            if self.handle_resize {
                if event::poll(Duration::from_millis(25)).expect("unable to pull events") {
                    if let Ok(Event::Resize(_, _)) = event::read() {
                        clear_screen();
                    }
                }
            }

            if !self.plain {
                self.fancy_file_pos_2();
            }
            
            if self.steps {
                thread::sleep(Duration::from_millis(25));
            }

            match self.curr_token.token_type() {
                TokenType::Start => {
                    //self.print_curr();
                    self.get_next();
                },
                TokenType::Str(string) => {
                    self.handle_output(format!("{} {}", "~".bold(), string.green()).green());
                    self.get_next();
                },

                TokenType::Input => {

                    let mut line = String::new();
                    println!("{}", "(Input)".black().bold().on_cyan());
                    let b1 = std::io::stdin().read_line(&mut line).unwrap();  
                    self.handle_output(format!("{}", line).yellow());
                    match format!("{}", line).chars().nth(0) {
                        Some(c) => self.stack.push(c as i32),
                        None => self.print_err("no characters inputed")
                    }

                    self.get_next();
                },
                TokenType::RandomNum => {
                    if self.stack.len() < 0 {
                        self.print_err("cannot get random without an upperbound which is the pointed value on the stack with nothing in the stack");
                        return;
                    }
                    let num = Self::get_rand(self.stack[self.stack_index]);
                    self.stack.push(num);
                    self.get_next();
                    self.handle_output(num.to_string().into());
                },
                TokenType::PrintChar => {
                    if self.stack.len() <= 0 {
                        self.print_err("cannot print char with no pointed to value [nothing on the stack]");
                        return;
                    }
                    let index:u32 = self.stack[self.stack_index] as u32;
                    let res:String = char::from_u32(index).expect("couldn't convert tos tring").to_string();

                    self.get_next();
                    self.handle_output(res.into());
                },
                TokenType::NumInput => {

                    let mut line = String::new();
                    println!("{}", "(Input)".black().bold().on_cyan());
                    let b1 = std::io::stdin().read_line(&mut line).unwrap();  
                    self.handle_output(format!("{}", line).normal());
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
                    self.handle_output(self.stack[self.stack_index].to_string().normal());
                    self.get_next();
                },
                TokenType::RightShift => {
                    self.stack_index = if self.stack_index + 1 >= self.stack.len() {0} else {self.stack_index + 1};

                    self.get_next();
                },
                TokenType::LeftShift => {
                    self.stack_index = if self.stack_index - 1 < 0 {if (self.stack.len() as i32- 1) < 0 {0} else {self.stack.len() - 1}} else {(self.stack_index as i32 - 1).try_into().unwrap()};

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
        clear_screen();
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

    fn fancy_file_pos_2(&mut self) {
        let mut index:usize = 0;
        let mut c = self.contents.chars().nth(index).unwrap();
        let mut line = 1;
        let mut colmn = 0;
        let mut output = String::from("");
        let size = terminal::size().unwrap();

        if self.tick_count > 0 {
            execute!(io::stdout(), cursor::MoveUp(self.output_height as u16 + size.1), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        }
        if self.tick_count % 1000 == 0{
            clear_screen();
        }
        self.tick_count += 1;

        // Move the cursor to the top of the terminal (row 1, column 1)
        print!("\x1B[H");  // ANSI escape code to move cursor to top-left corner
    
                          
    
        io::stdout().flush().unwrap();
    
        if self.show_flags {
            print!("\n== [{}], != [{}], < [{}], > [{}], <= [{}], >= [{}], ! [{}]\n", 
                    Self::fancy_bool(self.eq_flag), 
                    Self::fancy_bool(self.neq_flag), 
                    Self::fancy_bool(self.lt_flag), 
                    Self::fancy_bool(self.gt_flag), 
                    Self::fancy_bool(self.lteq_flag), 
                    Self::fancy_bool(self.gteq_flag), 
                    Self::fancy_bool(self.not_flag)
            
            );
        }

        //printing stack 
        let stack_top = if self.stack_index as i32 - 10 < 0 {0} else {self.stack_index - 10};

        let offset = self.stack.len()-self.stack_index;
        let stack_bottom = if offset > 10 {offset/10 * offset%10} else {self.stack.len()};

        /*output.push_str(&format!("\n{}\n[{}", "(Stack)".black().bold().on_cyan(), if stack_top == 0 {""} else {"..., "}));
        for i in stack_top..stack_bottom {
            if i == self.stack_index {
                output.push_str(&format!(" {},", self.stack[i].to_string().cyan().bold()));
                continue;
            }
            output.push_str(&format!(" {},", self.stack[i].to_string()));
        }
  //      output.push_str(&format!("]\n"));
        */
        if self.show_stack {
            print!("\n{}\n[{}", "(Stack)".black().bold().on_cyan(), if stack_top == 0 {""} else {"..., "});
            for i in stack_top..stack_bottom {
                if i == self.stack_index {
                    print!(" {},", self.stack[i].to_string().cyan().bold());
                    continue;
                }
                print!(" {},", self.stack[i].to_string());
            }
            print!("]\n");
        }

        if self.show_visualizer {
            //printing code body
            while c != '\0' {
                colmn += 1;
                let res = match c { 
                    'S' => "S".green().bold(),
                    // stack related commands
                    '>' => ">".yellow().bold(),
                    '<' => "<".yellow().bold(),
                    ',' => ",".yellow().bold(),
                    //alphanumeric commands
                    'P' => "P".bold(),
                    'C' => "C".bold(),
                    'W' => "W".bold(),
                    '#' => "#".yellow().bold(),
                    '~' => "~".blue().bold(),
                    '@' => "@".red().bold(),
                    '+' => "+".yellow().bold(),
                    '-' => "-".yellow().bold(),
                    'i' => "i".yellow().bold(),
                    'd' => "d".yellow().bold(),
                    '0' => "0".blue().bold(),
                    ' ' => " ".normal(),

                    '\t' => {
                        colmn += 4;
                        "    ".normal()
                    },
            
                    '\n' => {
                        colmn = 1;
                        line += 1;
                        "\n".normal()
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

                        let inner = self.contents[start as usize..index as usize].to_string().green().bold();
                        colmn += 1;
                        index -=1;
                        inner
                 
                    }
                    //computational
                    '?' => "?".blue().bold(),
                    '\0' => return,
                    _ => format!("{}", c).normal()
                };  
                if self.curr_token.index() == index { 
                    print!("{}", res.on_cyan());
                }else {

                    print!("{}", res);
                    //output.push_str(&res);
                }
                index += 1;
                match self.contents.chars().nth(index) {
                    Some(ch) => c=ch,
                    None => break
         
                };
            }
        }

        let (_, height) = terminal::size().expect("cannot get (width, height) of terminal using crossterm for some reason");
        let (_, row) = cursor::position().expect("cannot get (col, row) of terminal cursor using crossterm for some reason");
        let max_out:usize = if  ((height as i32-row as i32) - 2) < 0 {0} else {((height - row) - 2).into()};
        let top = if self.outputs.len() >= max_out && max_out != 0 { 
            ((self.outputs.len()/max_out) * max_out) as usize
        } else {0};

        if  max_out != 0{
            if top % max_out == 0 {
                let _ = stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown));
            }
        }

        if self.show_output {
            print!("{}",
                if top == 0 {
                    format!("{}{}", String::from("Terminal Output").black().on_cyan(), String::from("\n").on_cyan())
                }else {
                    format!("{}{}\n(...{} additional outputs)", String::from("Terminal Output").black().on_cyan(), String::from("\n").on_cyan(), top.to_string().green().bold())
                }
            );
            // Clear from the current cursor position down to the end of the line
            for out in top..self.outputs.len() {
                print!("\n{} {}", String::from("~").green().bold(), self.outputs[out]);
            }
            print!("{}", output);
        }
    }

}

