### `Command and chat handler for rust`

#### User-friendly bot-handler module for rust

#### Can be used for bots

## Module available at -

- [crates.io](https://crates.io/crates/bot-handler-manager)
- [docs.rs](https://docs.rs/bot-handler-manager/)

## Example

#### Cargo.toml

Add this to your cargo.toml file:

```toml
   bot-handler-manager = "0.1.2"
```

#### main.rs

```rust
// use
use bot_handler_manager;

fn main() {
   bot_handler_manager::text_speech_text("hello!");
     let mut commander = bot_handler_manager::Commander::new("You~> ");
     commander.on("hi", "Hello there!");
     // Loop and read
     loop {
         commander.read();
     }
}


```

Feel free to open issues and contribute!

#### breadA#3012

### This repo is licensed under:

- MIT
