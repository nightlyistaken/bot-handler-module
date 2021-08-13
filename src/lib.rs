use tts_rust::text_speech_text;
use std::io;
use std::io::Write;

/// Prompts on the CLI
pub fn readline(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut user_input = String::new();
    
    io::stdin().read_line(&mut user_input).unwrap();

    user_input
    
}

/// Prompt manager: 
pub fn if_msg(prefix :&str ,input: &str, output: &str) {
    let inside = readline(&prefix );
    if inside.contains(&input) {
        text_speech_text(&output);
    }
    
}

#[test]
fn test() {
    //' -  ' = This is will indicate the input is being typed at this line
    //' Hello, World! ' = Message to be typed by user
    //' Hey! I'm a bot! ' = Output of the message typed by user
    // Output will be --

    // - Hello, World!
    //   Hey! I'm a bot!
    
    if_msg("- ","Hello, World!", "Hey! I'm a bot!");
}
