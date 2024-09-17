#![feature(trait_alias)]

mod ast;
mod interperter;
mod lexer;
mod parser;
mod utils;

use anyhow;
use interperter::interperter::Interperter;
use lexer::lexer::scan;
use parser::parser::{get_stream, parse};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let input = match args.len() {
        0 => get_prompt(),
        _ => read_input(args.get(0).unwrap_or(&String::new()).to_string()),
    };
    run(&input)?;
    Ok(())
}

fn read_input(contents: String) -> String {
    let res = std::fs::read_to_string(contents);
    match res {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading file! with error {e}");
            String::new()
        }
    }
}

fn get_prompt() -> String {
    print!(">");
    let mut prompt = String::new();
    let res = std::io::stdin().read_line(&mut prompt);
    match res {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    prompt
}

fn run<'a>(input: &str) -> anyhow::Result<()> {
    let tokens = scan(input)?;
    let stream = get_stream((tokens, input));
    let stmt = parse(stream);
    let mut interperter = Interperter::default();
    interperter.stmt_eval(stmt)?;
    return Ok(());
}
