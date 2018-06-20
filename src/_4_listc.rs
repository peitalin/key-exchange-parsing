

use std::fs::File;
use std::io::{ Write, BufReader, BufRead };
use std::collections::HashMap;

pub fn listc() {

    let words: Vec<_> = BufReader::new(File::open("/usr/share/dict/words")
        .unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect();

    let words_length: HashMap<usize, _> = words.iter().map(|word| (word.len(), word)).collect();
    println!("{:?}", words_length);
}

