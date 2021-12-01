use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0")]
pub struct AdventOfCode {
    #[clap(short, long)]
    pub filename: String,
    
    #[clap(short, long)]
    pub day: i32
}