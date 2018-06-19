use std::fmt;

pub enum Language {
    English,
    German,
}

pub struct Greeter {
    language: Language,
}

impl fmt::Display for Greeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let greeting = match self.language {
            Language::English => "Hello",
            Language::German => "Hallo",
            _ => "Oi"
        };
        write!(f, "{} Rust", greeting)
    }
}

impl Greeter {
    pub fn new() -> Greeter {
        Greeter {
            language: Language::English,
        }
    }

    pub fn with_language(mut self, language: Language) -> Greeter {
        // must pass in mut self, to be able to modify this self instance
        self.language = language;
        self
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let greeter = Greeter::new().with_language(Language::German);
        assert_eq!(format!("{}", greeter), "Hallo Rust");
    }
}
