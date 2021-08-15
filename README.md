## Command and chat handler for bots

#### User-friendly bot-handler module for rust

#### Can be used for bots

## Module available at -

- [crates.io](https://crates.io/crates/bot-handler-manager)
- [docs.rs](https://docs.rs/bot-handler-manager/)

## Example

#### Cargo.toml

Add this to your cargo.toml file:

```toml
   bot-handler-manager = "0.1.3"
```

#### main.rs

```rust
// Use the crate
use bot_handler_manager;

fn main() {
    // Make a prefix
     let mut commander = bot_handler_manager::Commander::new("~> ");

    // Listen for messages/commands
     commander.on("Hello!", |_| {"Hello there!"}.to_string());

    // loop the readline()
     loop {
         // read 
         commander.read(Some(|input: &str| {
             // if the user types a message/command, and it contains in the vector,
             // the user can type in MIX-CASE.

            //  example: "HeLLO, RuST!"

            let my_user_input = vec!["hello, rust!", "hello, world!", "hello, bot!", "hi!"];

            if my_user_input.contains(&input) {
                String::from("Hello, I'm Alive!")
             } else {
                 String::from(" ")
             }
         }));
     }
}


```

### `Feel free to open issues and contribute!`

### Authors

- `breadA#3012`

### This crate and repo is licensed under:

- MIT

Thanks! :smiley:
