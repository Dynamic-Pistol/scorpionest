#![feature(trait_alias)]

pub mod scorpiodata;

use anyhow;
use scorpiodata as Data;

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
    let tokens = Data::scan(input)?;
    let stream = Data::get_stream((tokens, input));
    let stmt = Data::parse(stream);
    Data::stmt_eval(stmt)?;
    return Ok(());
}
