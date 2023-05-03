use std::{fs, collections::HashMap};

type Position = (i32, i32);

fn update_pos(pos: &mut Position, ch: &char) {
    match ch {
       '^' => pos.1 += 1,
       '>' => pos.0 += 1,
       'v' => pos.1 -= 1,
       '<' => pos.0 -= 1,
       _ => unreachable!()
    };
}

fn count(input: &str) -> i32 {
    let mut pos = (0,0);
    let mut robo_pos = (0,0);

    let mut seen: HashMap<Position, i32> = HashMap::from([(pos, 2)]);

    for (i, ch) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            update_pos(&mut robo_pos, &ch);
            seen.entry(robo_pos).and_modify(|c| *c += 1).or_insert(1);
        } else {
            update_pos(&mut pos, &ch);
            seen.entry(pos).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    return seen.len() as i32;
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();

    println!("{:?}", count(&input));
}
