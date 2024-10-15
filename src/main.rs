use std::char;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("\nInvalid argument format!\nUsage: json_parser [FILE_NAME]\n");
    }

    let content = fs::read_to_string(&args[1]).expect("Unable to read json file");
    match parse(&content) {
        Ok(_) => print!("VALID JSON!"),
        Err(_) => println!("INVLAID JSON!"),
    }
}

fn parse(content: &str) -> Result<(), ()> {
    if content.is_empty() {
        return Err(());
    }

    let mut stack: Vec<char> = Vec::new();
    let mut special_chars = HashMap::new();
    special_chars.insert('}', '{');
    special_chars.insert('"', '"');
    special_chars.insert(']', '[');
    let special_chars_close = Vec::from_iter(special_chars.keys());
    let special_chars_open = Vec::from_iter(special_chars.values());

    for character in content.chars() {
        if special_chars_open.contains(&&character) {
            stack.push(character.clone());
        } else if special_chars_close.contains(&&character) {
            if let Some(open) = special_chars.get(&character) {
                let last = match stack.last() {
                    Some(x) => x,
                    None => return Err(()),
                };

                if last != open {
                    return Err(());
                } else {
                    stack.pop();
                }
            }
        }
    }

    Ok(())
}
