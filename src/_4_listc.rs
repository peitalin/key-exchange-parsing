

use std::fs::File;
use std::io::{ Write, BufReader, BufRead };
use std::collections::HashMap;

pub fn listc() {

    let words: Vec<_> = BufReader::new(File::open("/usr/share/dict/words")
        .unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect();

    let words_length: HashMap<_, usize> = words.iter().map(|word| (word, word.len())).collect();
    println!("{:?}", words_length);
}

