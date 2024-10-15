use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("\nInvalid argument format!\nUsage: json_parser [FILE_NAME]\n");
    }

    let mut content = fs::read_to_string(&args[1]).expect("Unable to read json file");
    match parse_object(&mut content) {
        Ok(_) => print!("VALID JSON!"),
        Err(_) => println!("INVLAID JSON!"),
    }
}

fn parse_object(content: &mut str) -> Result<(), ()> {
    if content.is_empty() {
        return Err(());
    }

    let mut items = Vec::new();
    let mut special_chars_stack = Vec::new();
    let mut special_chars = HashMap::new();
    special_chars.insert('}', '{');
    special_chars.insert(']', '[');
    let special_chars_close = Vec::from_iter(special_chars.keys());
    let special_chars_open = Vec::from_iter(special_chars.values());

    let content = content.trim();
    if !content.starts_with('{') || !content.ends_with('}') {
        return Err(());
    }

    let content = (&content[1..content.len() - 1]).trim();
    let mut current_start = 0;

    for (i, character) in content.chars().enumerate() {
        if special_chars_open.contains(&&character) {
            special_chars_stack.push(character.clone());
        } else if special_chars_close.contains(&&character) {
            if let Some(open) = special_chars.get(&character) {
                let last = match special_chars_stack.last() {
                    Some(x) => x,
                    None => return Err(()),
                };

                if last != open {
                    return Err(());
                } else {
                    special_chars_stack.pop();
                }
            }
        }

        if special_chars_stack.is_empty() {
            if character == ',' {
                items.push((&content[current_start..i]).trim());
                current_start = i + 1;
            }
            if i == content.len() - 1 {
                let last_item = (&content[current_start..=i]).trim();
                if last_item.is_empty() {
                    return Err(());
                }
                items.push(last_item);
            }
        }
    }

    for item in items {
        let (key, _) = match item.split_once(':') {
            Some(x) => x,
            None => {
                return Err(());
            }
        };

        parse_string(key)?;
    }

    Ok(())
}

fn parse_string(content: &str) -> Result<(), ()> {
    if content.is_empty() || !content.starts_with('"') || !content.ends_with('"') {
        return Err(());
    }

    Ok(())
}
