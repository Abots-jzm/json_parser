use std::collections::HashMap;
use std::env;
use std::fs;

enum ObjectType {
    Object,
    Array,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("\nInvalid argument format!\nUsage: json_parser [FILE_NAME]\n");
    }

    let mut special_chars = HashMap::new();
    special_chars.insert('}', '{');
    special_chars.insert(']', '[');
    let special_chars_open = Vec::from_iter(special_chars.values());
    let special_chars_close = Vec::from_iter(special_chars.keys());

    let content = fs::read_to_string(&args[1]).expect("Unable to read json file");
    let content = content.trim();

    match parse_object(
        &content,
        ObjectType::Object,
        &special_chars,
        &special_chars_open,
        &special_chars_close,
    ) {
        Ok(_) => print!("VALID JSON!"),
        Err(_) => println!("INVLAID JSON!"),
    }
}

fn parse_object(
    content: &str,
    object_type: ObjectType,
    special_chars: &HashMap<char, char>,
    special_chars_open: &Vec<&char>,
    special_chars_close: &Vec<&char>,
) -> Result<(), ()> {
    if content.is_empty() {
        match object_type {
            ObjectType::Object => {
                if !content.starts_with('{') || !content.ends_with('}') {
                    return Err(());
                }
            }
            ObjectType::Array => {
                if !content.starts_with('[') || !content.ends_with(']') {
                    return Err(());
                }
            }
        }

        return Err(());
    }

    let mut items = Vec::new();
    let mut special_chars_stack = Vec::new();

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
        let value = match object_type {
            ObjectType::Object => {
                let (key, value) = match item.split_once(':') {
                    Some((key, value)) => (key.trim(), value.trim()),
                    None => {
                        return Err(());
                    }
                };

                parse_string(key)?;
                value
            }
            ObjectType::Array => item.trim(),
        };

        if value.starts_with('{') || value.starts_with('[') {
            parse_object(
                &value,
                if value.starts_with('{') {
                    ObjectType::Object
                } else {
                    ObjectType::Array
                },
                special_chars,
                special_chars_open,
                special_chars_close,
            )?
        } else if value.starts_with('"') {
            parse_string(value)?
        } else {
            parse_other(value)?
        }
    }

    Ok(())
}

fn parse_other(content: &str) -> Result<(), ()> {
    if content.is_empty() {
        return Err(());
    }

    if content == "true" || content == "false" || content == "null" {
        return Ok(());
    }

    if let Ok(_) = content.parse::<i64>() {
        return Ok(());
    }

    // If that fails, try parsing as a float
    if let Ok(_) = content.parse::<f64>() {
        return Ok(());
    }

    Err(())
}

fn parse_string(content: &str) -> Result<(), ()> {
    if content.is_empty() || !content.starts_with('"') || !content.ends_with('"') {
        return Err(());
    }

    Ok(())
}
