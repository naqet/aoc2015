use std::{fs, collections::HashMap};

fn check_pairs(input: &str) -> bool {
    let mut valid = false;

    let mut doubles: HashMap<String, i32> = HashMap::new();

    let chars: Vec<char> = input.chars().collect();


    for (i, _) in input.chars().enumerate() {
        if i == input.len() - 1 {
            break;
        }
        let double = format!("{}{}", chars[i], chars[i + 1]);
        doubles.entry(double).and_modify(|count| *count += 1).or_insert(1);
    }

    for (value, _) in doubles.into_iter().filter(|(_, count)| *count >= 2) {
        let matcher: Vec<&str> = input.matches(&value).collect();

        if matcher.len() >= 2 {
            valid = true;
            break;
        }
    }

    return valid;
}

fn check_between(input: &str) -> bool {
    let mut valid = false;
    let chars: Vec<char> = input.chars().collect();

    for (i, c) in input.chars().enumerate() {
        if i < 2 {
            continue;
        }
        if c == chars[i - 2] {
            valid = true;
            break;
        }
    }
    return valid;

}

fn main() {
    let input = fs::read_to_string("src/input").expect("File cannot be read");
    
    let mut valid_count = 0;

    for line in input.lines() {
        if check_between(line) && check_pairs(line) {
            valid_count += 1;
        }
    }
    println!("{valid_count}");
}
