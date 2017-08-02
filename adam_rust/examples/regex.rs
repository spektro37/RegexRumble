extern crate regex;

use std::env;
use regex::Regex;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Needs 2 arguments");
        return;
    }
    let r = Regex::new(&args[1]);
    if !r.is_match_str(&args[2]) {
        println!("Did not match '{}' with '{}'", args[1], args[2]);
    } else {
        println!("Matched '{}' with '{}'!", args[1], args[2]);
    }
}
