

pub enum Language {
    English,
    German,
}

pub fn greet(language: Language) {
    let greeting = match language {
        Language::English => "Hello",
        Language::German => "Hallo",
        _ => "Oi"
    };
    println!("{} Rust", greeting);
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
