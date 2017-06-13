extern crate clap;
extern crate erl_pp;
extern crate erl_tokenize;
#[macro_use]
extern crate trackable;

use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use clap::{App, Arg};
use erl_pp::Preprocessor;
use erl_tokenize::{Lexer, PositionRange};

fn main() {
    let matches = App::new("pp")
        .arg(Arg::with_name("SOURCE_FILE").index(1).required(true))
        .arg(Arg::with_name("SILENT").long("silent"))
        .get_matches();
    let src_file = matches.value_of("SOURCE_FILE").unwrap();
    let silent = matches.is_present("SILENT");

    let mut src = String::new();
    let mut file = File::open(src_file).expect("Cannot open file");
    file.read_to_string(&mut src).expect("Cannot read file");

    let start_time = Instant::now();
    let mut count = 0;
    let lexer = Lexer::new(&src);
    let preprocessor = Preprocessor::new(lexer);
    for result in preprocessor {
        let token = track_try_unwrap!(result);
        if !silent {
            println!("[{:?}] {:?}", token.start_position(), token.text());
        }
        count += 1;
    }
    println!("TOKEN COUNT: {}", count);
    println!("ELAPSED: {:?} seconds",
             to_seconds(Instant::now() - start_time));
}

fn to_seconds(duration: Duration) -> f64 {
    duration.as_secs() as f64 + (duration.subsec_nanos() as f64) / 1_000_000_000.0
}
