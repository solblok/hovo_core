use regex::Regex;
use std::process::Command;

pub fn format_response(text: &str) -> String {
    let no_emoji = Regex::new(r"[^\p{L}\p{N}\p{P}\p{Zs}\n]").unwrap().replace_all(text, "");
    let no_bold = Regex::new(r"\*\*(.*?)\*\*").unwrap().replace_all(&no_emoji, "$1");
    no_bold.to_string().replace("[0m", "")
}

pub fn say(text: &str) {
    Command::new("espeak")
        .args(&["-v", "es", text]) 
        .spawn()
        .expect("Error ejecutando espeak");
}
