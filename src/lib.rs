

#![allow(dead_code)]
use tts_rust::text_speech_text;
use std::collections::HashMap;
use std::io::Write;
use std::io;
pub struct Commander {
    prefix: String,
    handlers: HashMap<String, String>,
}

 impl Commander {
    pub fn new(prefix: &str) -> Commander {
        Commander {
            prefix: prefix.to_string(),
            handlers: HashMap::new(),
        }
    }

    pub fn on(&mut self, key: &str, value: &str) {
        self.handlers.insert(key.to_string(), value.to_string());
    }

    pub fn read(&self) {
        let input = readline(&self.prefix);
        let input = input.trim();
        if self.handlers.contains_key(input) {
            let answer = self.handlers.get(input).unwrap();
            text_speech_text(answer);
        }
    }
}

/// Prompts on the CLI
pub fn readline(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input
    
}

#[test]
fn test() {
     
     text_speech_text("hello!");
     let mut commander = Commander::new("~> ");
     commander.on("hi", "Hello");
     // Commands section
     loop {
         commander.read();
     }
}


