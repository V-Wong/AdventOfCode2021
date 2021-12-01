use clap::Parser;

mod command_line_parser;
mod day1;
mod file_parser;

use command_line_parser::AdventOfCode;
use file_parser::read_lines as parse;

fn main() -> Result<(), String> {
    let commands = AdventOfCode::parse();

    let day = commands.day;
    let filename = commands.filename;

    Ok(match commands.day {
        0 => println!("{:?}", parse(&filename, |x| Some(x))),
        1 => {
            let parsed_contents = parse(&filename, day1::parse).ok_or("Could not parse file")?;
            println!(
                "{:} {:}",
                day1::solve1(&parsed_contents),
                day1::solve2(&parsed_contents)
            );
        }
        _ => panic!("No solution exists for day {:?}", day),
    })
}
