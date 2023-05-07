use std::{fs, collections::HashMap};

fn get_value(input: &str, hm: &mut HashMap<String, u16>) -> Option<u16> {
    if input.parse::<u16>().is_ok() {
        return Some(input.parse::<u16>().unwrap());
    } else {
        return hm.get(input).copied();
    }
}

fn process_data(input: &str, hm: &mut HashMap<String, u16>) -> Option<u16> {
    let command: Vec<&str> = input.split_whitespace().collect();

    match command.len() {
        3 => {
            let value = get_value(command[0], hm)?; 
            hm.insert(command[2].to_owned(), value);
        },
        4 => {
            let value = get_value(command[1], hm)?; 
            hm.insert(command[3].to_owned(), !value);
        },
        5 => {
            let value1 = get_value(command[0], hm)?; 
            let value2 = get_value(command[2], hm)?;
            hm.insert(
                command[4].to_owned(), 
                match command[1] {
                    "AND" => value1 & value2,
                    "OR" => value1 | value2,
                    "RSHIFT" => value1 >> value2,
                    "LSHIFT" => value1 << value2,
                    _ => panic!("What")
                }
            );
        },
        _ => panic!("Whaaat")
    }

    return Some(0);
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut hm: HashMap<String, u16> = HashMap::new();

    let mut lines: Vec<&str> = input.lines().collect();

    let mut i = 0;

    while !lines.is_empty() {
        hm.insert("b".to_owned(), 956);
        match process_data(lines[i], &mut hm) {
            Some(_) => {
                lines.remove(i);
            },
            None => {
                i += 1;
            },
        };

        if i >= lines.len() {
            i = 0;
        }
    }

    println!("{:?}", hm.get("a"));
}
