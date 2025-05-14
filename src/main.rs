//Created by: Nisha Murali (Requested not to clone / re-use the code)
//main.rs (CLI Loop and entry point) 
mod tokenizer;
mod parser;
mod pratt;
mod error;
mod ast;


use std::io::{self, Write};
use tokenizer::{Tokenizer, Token};
use parser::SQLParser;

fn main() 
{
    println!("🔷 Welcome to Basic SQL Parser CLI By Nisha Murali");
    println!("Type a SQL query or 'exit' to quit.\n");

    loop 
    {
        print!("sql> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() 
        {
            eprintln!("Failed to read input.");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") 
        {
            println!("👋Thank you for using, Goodbye!");
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        loop 
        {
            let token = tokenizer.next_token();
            tokens.push(token.clone());
            if token == Token::Eof 
            {
                break;
            }
        }

        let mut parser = SQLParser::new(&tokens);
        match parser.parse_statement() 
        {
            Ok(statement) => println!("\n✅ Parsed Statement:\n{:#?}\n", statement),
            Err(e) => eprintln!("❌ Error: {}\n", e),
        }
    }
}
