use std::io::{stdin, stdout, Result, Write};

use moca::lexer::Lexer;

pub fn start() {
    println!(":: start repl ::");

    loop {
        match input().as_deref() {
            Ok("exit") => break,
            Ok(line) => eval(line),
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    println!(":: end repl ::");
}

fn eval(line: &str) {
    let mut lexer = Lexer::new(line);

    while let Some(token) = lexer.next_token() {
        println!("{}", token.to_string());
    }
}

fn input() -> Result<String> {
    print!(">> ");
    stdout().flush()?;

    let mut buf: String = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim_end().to_string())
}
