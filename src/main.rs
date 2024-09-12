#![feature(trait_alias)]

pub mod scorpiodata;

use anyhow;
use lasso::Rodeo;
use scorpiodata as Data;

fn main() -> anyhow::Result<()> {
    let mut rodeo = Rodeo::default();
    let spur = rodeo.try_get_or_intern("test").unwrap();
    let s = rodeo.resolve(&spur);
    println!("{s}");
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

fn run(input: &str) -> anyhow::Result<()> {
    // Report::build(ariadne::ReportKind::Error, "test.scorp", 12)
    //     .with_code(3)
    //     .with_message(format!("Invalid error!"))
    //     .with_label(Label::new(1..2));
    let new_rodeo = Rodeo::default();
    let var_name = Data::NeededItems { rodeo: &new_rodeo };
    let mut items = var_name;
    let tokens = Data::scan(input)?;
    let stream = Data::get_stream((tokens, input));
    let stmt = Data::parse(stream, items);
    Data::stmt_eval(stmt.clone())?;
    return Ok(());
}
