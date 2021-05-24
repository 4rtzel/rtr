use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod parser {
    pub mod grid_slice_parser;
}

mod grid_slice;

#[derive(Debug)]
struct Opts {
    program: String,
    file: Option<String>,
}

fn parse_args() -> Result<Opts, &'static str> {
    let mut args = env::args().skip(1);

    Ok(Opts {
        program: args.next().ok_or("Wrong number of arguments")?,
        file: args.next(),
    })
}

struct SplitLines<I: BufRead> {
    source: I,
}

impl<I: BufRead> SplitLines<I> {
    fn new(source: I) -> Self {
        SplitLines { source: source }
    }
}

impl<I: BufRead> Iterator for SplitLines<I> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        if self.source.read_line(&mut line).ok()? == 0 {
            None
        } else {
            Some(line.split_whitespace().map(|f| f.to_string()).collect())
        }
    }
}

fn main() -> Result<(), &'static str> {
    let opts = parse_args()?;
    let grid_slice = parser::grid_slice_parser::parse_grid_slice(&opts.program)?;
    match opts.file {
        Some(f) => {
            let file = File::open(f).or(Err("Unable to open a file"))?;
            let reader = BufReader::new(file);

            for line in grid_slice::grid_slice_iter(grid_slice, SplitLines::new(reader)) {
                println!("{}", line.join(" "));
            }
        }
        None => {
            let stdin = io::stdin();
            for line in grid_slice::grid_slice_iter(grid_slice, SplitLines::new(stdin.lock())) {
                println!("{}", line.join(" "));
            }
        }
    }

    Ok(())
}
