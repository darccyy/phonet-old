use std::fs;

use clap::Parser;
use phoner::{run_tests, Args, Scheme};

fn main() {
  let args = Args::parse();

  let file = fs::read_to_string(&args.file).expect(&format!("Could not read file '{}'", args.file));

  let scheme = Scheme::parse(&file).expect("Could not parse file");

  run_tests(scheme);
}
