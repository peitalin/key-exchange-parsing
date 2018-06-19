

pub enum Language {
    English,
    German,
}

pub fn greet(language: Language) {
    match language {
        Language::English => println!("Hello Rust"),
        Language::German => println!("Hallo German"),
        _ => println!("Nothing ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet(Language::English)
    }
    fn it_works_ja() {
        greet(Language::English)
    }
}
