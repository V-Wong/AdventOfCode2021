mod command_line_parser;

use clap::Parser;
use command_line_parser::AdventOfCode;

fn main() {
    let commands = AdventOfCode::parse();

    match commands.day {
        _ => panic!("No solution exists for day {:?}", commands.day)
    }
}
