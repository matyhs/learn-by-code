use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line_value = line.unwrap();

        match re.find(&line_value) {
            Some(_) => println!("{}", line_value),
            None => ()
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
                    .version("0.1")
                    .about("searches for patterns")
                    .arg(Arg::with_name("pattern")
                            .help("The pattern to search for")
                            .takes_value(true)
                            .required(true))
                    .arg(Arg::with_name("input")
                            .help("File to search")
                            .takes_value(true)
                            .required(true))
                    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let reg = Regex::new(pattern).unwrap();
    
    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, reg);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, reg);
    }

    
}
