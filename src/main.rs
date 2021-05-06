use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

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

fn main() -> Result<(), &'static str> {
    let opts = parse_args()?;
    let grid_slice = parser::grid_slice_parser::parse_grid_slice(&opts.program)?;
    match opts.file {
        Some(f) => {
            let file = File::open(f).or(Err("Unable to open a file"))?;
            let reader = BufReader::new(file);

            for line in grid_slice::grid_slice_iter(grid_slice, reader.lines()) {
                println!("{}", line);
            }
        },
        None => {
            let stdin = io::stdin();
            for line in grid_slice::grid_slice_iter(grid_slice, stdin.lock().lines()) {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
