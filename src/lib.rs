use tts_rust::text_speech;
use tts_rust::text_speech_text;
use std::io;
/// Prompts on the CLI
pub fn readline(msg: &str) -> String {
    println!("{}", msg);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

/// This will prompt
pub fn if_speech_out(prefix :&str ,input: &str, output: &str) {
    let inside = readline(&prefix );
    if inside.contains(&input) {
        text_speech_text(&output);
    }
    
}

#[test]
fn test() {
    if_speech_out("___","YO", "hi");
    if_speech_out("-------","HEY!", "yo lol");
}
