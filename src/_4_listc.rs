

use std::fs::File;
use std::io::{ Write, BufReader, BufRead };

pub fn listc() {

    let lines: Vec<_> = BufReader::new(File::open("/usr/share/dict/words")
        .unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect();

    println!("{:?}", lines);
}

