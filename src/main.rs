use clap::Parser;

mod command_line_parser;
mod file_parser;
mod day1;

use command_line_parser::AdventOfCode;
use file_parser::{read_lines as parse};

fn main() -> Result<(), String> {
    let commands = AdventOfCode::parse();

    let day = commands.day;
    let filename = commands.filename;

    Ok(match commands.day {
        0 => println!("{:?}", parse(&filename, |x| Some(x))),
        _ => panic!("No solution exists for day {:?}", day),
    })
}
