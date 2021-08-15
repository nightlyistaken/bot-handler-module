//! Welcome to bot-handler-managing module!
//! This module is User-friendly, Fell free to use!
// `                                              `
//! See the Docs for documentation

#![allow(dead_code)]
use tts_rust::text_speech_text;
use std::collections::HashMap;
use std::io::Write;
use std::io;
use std::boxed::Box;

/// Make a Struct commander with prefix and handler(s).
pub struct Commander {
    prefix: String,
    handlers: HashMap<String, Box<dyn Fn(String) -> String>>,
}

 impl Commander {
     /// Creates a new prefix (string)
    pub fn new(prefix: &str) -> Commander {
        Commander {
            prefix: prefix.to_string(),
            handlers: HashMap::new(),
        }
    }
    /// Creates a new input and output for the command/message
    pub fn on(&mut self, key: &str, handler: fn(String) -> String) {
        self.handlers.insert(key.to_string(), Box::new(handler));
    }

    /// Read those "commander.on()" messages/commands...
    pub fn read(&self, global_handler: Option<fn(&str) -> String>) {
        let input = readline(&self.prefix).to_lowercase();
        let input = input.trim();
        if self.handlers.contains_key(input) {
            let answer = self.handlers.get(input).unwrap();
            text_speech_text(&answer(input.to_string()));
        } else {
            match global_handler {
                Some(h) => {
                    text_speech_text(&h(input));
                },
                None => {}
            }
        }
    }
}

/// Prompts on the Command line, like in JS you "prompt()".
pub fn readline(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    return user_input;
    
}

#[test]
fn test() {
   
     text_speech_text("hello!");
     let mut commander = Commander::new("~> ");
     commander.on("hi", |_| {"Hello"}.to_string());
     // Commands section
     loop {
         commander.read(Some(|input: &str| {
            let my_user_input = vec!["hello, rust!", "hello, world!", "hello, bot!", "hi!"];
            if my_user_input.contains(&input) {
                String::from("Pudu")
             } else {
                 String::from(" ")
             }
         }));
     }
}


