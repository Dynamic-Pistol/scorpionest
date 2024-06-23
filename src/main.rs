#![feature(trait_alias)]

pub mod scorpiodata;
use scorpiodata as Data;
use std::io::Read;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let input = match args.len() {
        0 => run_prompt(),
        _ => read_input(args.get(0).unwrap_or(&String::new()).to_string()),
    };
    run(&input);
    std::io::stdin().read(&mut [0]).unwrap();
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

fn run_prompt() -> String {
    print!(">");
    let mut prompt = String::new();
    let res = std::io::stdin().read_line(&mut prompt);
    match res {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    prompt
}

fn run(input: &str) {
    let tokens = Data::scan(input);
    let stream = match tokens {
        Ok(t) => Data::get_stream((t, input)),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let stmt = Data::parse(stream);
    Data::stmt_eval(stmt.clone());
}
