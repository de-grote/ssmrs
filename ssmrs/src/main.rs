use std::{fs::read_to_string, path::PathBuf};

use chumsky::Parser as _;
use clap::{ArgAction, Parser};
use ssmrs::cpu::Cpu;

#[derive(Parser, Debug)]
#[clap(
    name = "ssmrs",
    author = "Julius de Jeu",
    about = "A simple stack machine"
)]
struct Cli {
    #[clap(help = "The file to run")]
    file: PathBuf,

    #[clap(short, long, action = ArgAction::Count, help = "Increase verbosity")]
    verbosity: u8,
}

fn main() {
    let res = Cli::parse();
    println!("{:?}", res.file);
    let code = read_to_string(res.file).unwrap();
    let c = ssmrs::parse().parse(code).unwrap();
    let mut cpu = Cpu::new(res.verbosity, Box::new(|s| println!("{}", s)));
    cpu.load_code(c);
    while cpu.step() {}
}
