use std::io::{stdin, stdout, Result, Write};

use moca::{lexer::Lexer, parser::Parser};

pub fn start() {
    println!(":: start repl ::");

    loop {
        match input().as_deref() {
            Ok("exit") => break,
            Ok(line) => repl(line),
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    println!(":: end repl ::");
}

fn repl(line: &str) {
    let lexer = Lexer::new(line);
    let mut parser = Parser::new(lexer);

    parser.parse_program();
}

fn input() -> Result<String> {
    print!(">> ");
    stdout().flush()?;

    let mut buf: String = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim_end().to_string())
}
