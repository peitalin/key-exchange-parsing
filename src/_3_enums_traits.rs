

pub enum Language {
    English,
    German,
}

pub struct Greeter {
    language: Language,
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

    pub fn greet(self) {
        let greeting = match self.language {
            Language::English => "Hello",
            Language::German => "Hallo",
            _ => "Oi"
        };
        println!("{} Rust", greeting);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let greeter = Greeter::new();
        greeter.greet();
    }
}
