use std::fs;

fn calculate_basement_index(input: &str) -> i32 {
    let mut floor_index = 0;
    let mut basement_index = 0;

    for (i, c) in input.trim().chars().into_iter().enumerate() {
        if c == '(' {
            floor_index += 1;
        } else if c == ')' {
            floor_index -= 1;
        }

        if floor_index == -1 {
            basement_index = i + 1;
            break;
        }
    }

    return basement_index as i32;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    println!("{}", calculate_basement_index(&input));
}
