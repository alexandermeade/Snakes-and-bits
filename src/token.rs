 
use colored::Colorize;



#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Start,
    Ladder,
    Snake,
    RightShift,
    LeftShift,
    Input,
    PopCmd,
    CopyCmd,
    Stop,
    Str(String),
    NA(char),
    FailedDelimiter(char),
    NewLine,
    Step,
    EOF,
    Zero,
    Add,
    Sub,
    Inc,
    Dec,
    Front,
    Back,
    PrintCell,
    LeftPan,
    RightPan,
    Compare,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
    EqualTo,
    NotEqualTo,
    NumInput,
    WipeCmd,
    Div,
    Mult,
    Mod
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    colmn:u32,
    line:u32,
    colmn_end:u32,
    line_end:u32
}

impl Token {
    pub fn new(token_type:TokenType, colmn:u32, line:u32) -> Token {
        Token {
            token_type,
            colmn,
            line,
            colmn_end: 0,
            line_end: 0
        }
    }
    
    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn print_err(&self, msg:String) {
        println!("{} at line {},  colmn {}", msg.red().bold(), self.line, self.colmn);
    }
    
    pub fn token_type(&self) -> TokenType {
        return self.token_type.clone();
    }

    pub fn line(&self) -> u32{
        return self.line;
    } 
    
    pub fn colmn(&self) -> u32{
        return self.colmn;
    } 

    pub fn line_end(&self) -> u32{
        return self.line_end;
    } 
    
    pub fn colmn_end(&self) -> u32{
        return self.colmn_end;
    } 
    
    pub fn set_ends(&mut self, colmn:u32, line:u32) {
        self.line_end = line;
        self.colmn_end = colmn;
    }
}



