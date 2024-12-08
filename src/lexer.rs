
use crate::token::Token;
use crate::token::TokenType;


pub struct Lexer {
    content: String,
    tokens: Vec<Token>,
    curr_char: char,
    line: u32,
    colmn: u32,
    index:usize
}

impl Lexer {
    
    fn next(&mut self) {
        if self.index + 1 >= self.content.len().try_into().unwrap() {
            self.curr_char = '\0';
            return;
        }

        match self.curr_char {
            '\n' => {
                self.line += 1;
                self.colmn = 1;
                self.index += 1;
            },
            '\t' => {
                self.colmn += 4;
                self.index += 1;
            },
            _ => {
                self.colmn += 1;
                self.index += 1;
            } 
        }
        self.curr_char = self.content.chars().nth(self.index).unwrap();
    }
    fn parse_str(&mut self) -> Token {
        //move past open qoute
        self.next();
        let start_colmn = self.colmn; 
        let start = self.index;
        while self.curr_char != '"' {
            if self.curr_char == '\0' {
                self.index = start + 1;
                return Token::new(TokenType::FailedDelimiter('"'), self.colmn, self.line) 
            }
            if self.curr_char == '\\' {
                self.next(); //skip the \
                self.next(); //skip the char after \
                continue;
            }
            self.next();
        }

        Token::new(TokenType::Str(self.content[start..self.index].to_string()), start_colmn, self.line)
    }
    fn lex(&mut self) -> Token {
        match self.curr_char {
            'S' => Token::new(TokenType::Start, self.colmn, self.line),
            ' ' | '\t'=> {
                self.next();
                self.lex()
            },
            '\n' => Token::new(TokenType::NewLine, self.colmn, self.line),
            '"' => {
                self.parse_str()
            },
            '@' => Token::new(TokenType::Stop, self.colmn, self.line),
            '#' => Token::new(TokenType::Ladder, self.colmn, self.line),
            '~' => Token::new(TokenType::Snake, self.colmn, self.line),
            '<' => Token::new(TokenType::LeftShift, self.colmn, self.line),
            '>' => Token::new(TokenType::RightShift, self.colmn, self.line),
            '0' => Token::new(TokenType::Zero, self.colmn, self.line),
            '\0' => Token::new(TokenType::EOF, self.colmn, self.line),
            '+' => Token::new(TokenType::Add, self.colmn, self.line),
            '-' => Token::new(TokenType::Sub, self.colmn, self.line),
            'i' => Token::new(TokenType::Inc, self.colmn, self.line),
            'd' => Token::new(TokenType::Dec, self.colmn, self.line),
            '[' => Token::new(TokenType::Front, self.colmn, self.line),
            ']' => Token::new(TokenType::Back, self.colmn, self.line),
            '.' => Token::new(TokenType::Step, self.colmn, self.line),
            '$' => Token::new(TokenType::PrintCell, self.colmn, self.line),
            ',' => Token::new(TokenType::Input, self.colmn, self.line),
            '?' => Token::new(TokenType::Compare, self.colmn, self.line),
            'l' => Token::new(TokenType::LessThan, self.colmn, self.line),
            'L' => Token::new(TokenType::LessThanEqualTo, self.colmn, self.line),
            'g' => Token::new(TokenType::GreaterThan, self.colmn, self.line),
            'G' => Token::new(TokenType::GreaterThanEqualTo, self.colmn, self.line),
            '=' => Token::new(TokenType::EqualTo, self.colmn, self.line),
            '!' => Token::new(TokenType::NotEqualTo, self.colmn, self.line),
            '_' => Token::new(TokenType::NumInput, self.colmn, self.line),
            'W' => Token::new(TokenType::WipeCmd, self.colmn, self.line),
            'C' => Token::new(TokenType::CopyCmd, self.colmn, self.line),
            'P' => Token::new(TokenType::PopCmd, self.colmn, self.line),
            '/' => Token::new(TokenType::Div, self.colmn, self.line),
            '*' => Token::new(TokenType::Mult, self.colmn, self.line),
            '%' => Token::new(TokenType::Mod, self.colmn, self.line),
            '\\' => Token::new(TokenType::LeftPan, self.colmn, self.line),
            _ => Token::new(TokenType::NA(self.curr_char), self.colmn, self.line)
        }
    }

    pub fn start(content:&str) -> Vec<Vec<Token>> {
        if content.len() <= 0 {
            return vec![];
        }

        let mut lexer = Lexer {
            content: content.to_string(),
            tokens: Vec::new(),
            curr_char: content.chars().nth(0).unwrap(),
            line: 1,
            colmn: 1,
            index: 0
        };

        while lexer.curr_char != '\0' {
            let tok = lexer.lex();
            lexer.tokens.push(tok);
            lexer.next();
        }
        lexer.print_errs();
        lexer.print_toks();
        
//        println!("{:#?}", lexer.tokens_as_board());
        
        //lexer.tokens
        lexer.tokens_as_board()

    }
    
    fn tokens_as_board(&self) -> Vec<Vec<Token>> {
        if self.tokens.len() <= 0 {
            return Vec::new();
        }
        let mut res:Vec<Vec<Token>> = Vec::new();
        let mut start = 0;
        let mut curr = 0;
        let mut curr_line = self.tokens[0].line();
        for token in &self.tokens {

            if curr_line != token.line() && curr < self.tokens.len() {
                let t = Token::new(TokenType::EOF, 0, 0);
                let mut vec = self.tokens[start..curr].to_vec();
                vec.push(t);
                res.push(vec);
                curr_line = token.line();
                start = curr;
            }
            curr += 1; 
        }
        return res;
    }

    pub fn print_errs(&self) {
         for token in &self.tokens {  
             match token.token_type() {
                 TokenType::NA(c) => token.print_err(format!("{} is unknown", c)),
                 TokenType::FailedDelimiter(c) => token.print_err(format!("can't find closing {}", c)),
                 _ => continue
             };
         }       
    }
    pub fn print_toks(&self) {
        for token in &self.tokens {
            match token.token_type() { 
                TokenType::NA(_) => continue,
                _ => token.print()
            }
        }
    }
} 





