#[macro_use]
extern crate lalrpop_util;

use std::io::Read;

use log::error;

mod syntax;
use crate::syntax::*;

fn main() {
    let config = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::new("INPUT")
                .help("Sets the input file (defaults to '-', meaning STDIN)")
                .default_value("-")
                .index(1),
        )
        .get_matches();

    let input_source = config.value_of("INPUT").expect("input source");
    let mut input = String::new();
    let res = if input_source == "-" {
        std::io::stdin().read_to_string(&mut input)
    } else {
        std::fs::File::open(input_source).and_then(|mut f| f.read_to_string(&mut input))
    };

    if let Err(err) = res {
        error!("I/O error: {}", err);
        std::process::exit(47);
    }

    let p = Program::parse(&input).unwrap_or_else(|e| {
        error!("Parse error:\n{}", e);
        std::process::exit(2);
    });

    for c in p.0 {
        println!("{:?}", c);
    }
}
