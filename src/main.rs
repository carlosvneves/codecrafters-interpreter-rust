use core::fmt;
use std::env;
use std::fs;
use std::io::{self, Write};

use std::process;

fn main() {
    

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ;     
    }

    let command = &args[1];
    let filename = &args[2];

    

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let _ =tokenize(&file_contents);        
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ;
        }
    }
}

// <token_type> <lexeme> <literal>
//, , ., -, +, ;, *. /
enum Token{
    EOF,
    RightParen,     
    LeftParen,
    RightBrace,
    LeftBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,


    // Dot,
    // Comma,
    // Plus,
    // Minus,
    // Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Tab,
    // Space
    // Identifier(String),
    // StringLiteral(String),
    // NumberLiteral(f64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::EOF => write!(f, "EOF  null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Less=> write!(f, "LESS < null"),
            Token::LessEqual=>write!(f, "LESS_EQUAL <= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual =>write!(f, "GREATER_EQUAL >= null"),

        }
    }
}


fn tokenize(lexeme: &str) -> Result<(), i32>{


    // TODO: 
    // implementar vector para fazer push dos tokens
    // refatorar código - DRY para a avaliação de ops compostos com '='
    let mut status:i32 = 0;
    let mut line = 1;
    let mut chars = lexeme.chars().peekable();
    while let Some(f) = chars.next() {
        match f {
            '(' => {
                println!("{}",Token::LeftParen);
            },
            ')' => {
                println!("{}",Token::RightParen);
            },
            '{' => {
                println!("{}", Token::LeftBrace);
            },
            '}' =>{
                println!("{}", Token::RightBrace);

            },
            ',' => {
                println!("{}", Token::Comma);
            },
            '.' =>{
                println!("{}", Token::Dot);
            },
            '-' =>{
                println!("{}", Token::Minus);
            },
            '+' =>{
                println!("{}", Token::Plus);
            },
            ';' =>{
                println!("{}", Token::Semicolon);
            },
            '*' =>{
                println!("{}", Token::Star);
            },
            '/' =>{
                if lexeme.chars().count() > 1{
                    if let Some('/') = chars.peek().cloned() {
                        
                        // while chars.next() != Some('\n'){
                        //     chars.next(); 
                        // }
                        // break;

                    } else {
                        println!("{}", Token::Slash);
                    }
                  } else {
                    println!("{}", Token::Slash);
                }
            },
            '=' => {
                if lexeme.chars().count() > 1{
                    if let Some('=') = chars.peek().cloned() {
                            chars.next(); // Consume the '='
                        println!("{}", Token::EqualEqual);
                    } else {
                        println!("{}", Token::Equal);
                    }
                  } else {
                    println!("{}", Token::Equal);
                }
                
            },
            '!'=>{
                if lexeme.chars().count() > 1{
                    if let Some('=') = chars.peek().cloned() {
                        chars.next();                         
                        println!("{}", Token::BangEqual);
                    } else {
                        println!("{}", Token::Bang);
                    }
                  } else {
                    println!("{}", Token::Bang);
                }
            },
            '>'=>{

                if lexeme.chars().count() > 1{
                    if let Some('=') = chars.peek().cloned() {
                        chars.next();                         
                        println!("{}", Token::GreaterEqual);
                    } else {
                        println!("{}", Token::Greater);
                    }
                  } else {
                    println!("{}", Token::Greater);
                }
            },
            '<'=>{
                if lexeme.chars().count() > 1{
                    if let Some('=') = chars.peek().cloned() {
                        chars.next();                         
                        println!("{}", Token::LessEqual);
                    } else {
                        println!("{}", Token::Less);
                    }
                  } else {
                    println!("{}", Token::Less);
                }

            },
            ' '=>{


            },
            '\t'=>{

            },
            '\n'=>{
                line+=1;

            },
            _ =>{
                eprintln!("[line {}] Error: Unexpected character: {}",line, f);
                status = 65;
                
            }
        };
    }
    println!("{}",Token::EOF);

    process::exit(status);
}

// fn check_composed_ops(prev: char, next: char, current: char) -> String{
//
//     retun format!("{}")
//
// } 
