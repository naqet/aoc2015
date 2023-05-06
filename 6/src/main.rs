use std::{fs, collections::HashMap};

#[derive(Debug)]
enum Command {
    On,
    Off,
    Toggle
}

struct Instruction {
    command: Command,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
}

fn get_turn(turn: &str) -> Command {
    if turn == "on" {
        return Command::On;
    } else {
        return Command::Off;
    }
}

fn get_coords(coords: &str) -> (i32, i32){
    let splitted: Vec<&str> = coords.split(",").collect();

    let x: i32 = splitted[0].parse().unwrap();
    let y: i32 = splitted[1].parse().unwrap();

    return (x, y);
}

fn get_instuction(input: &str) -> Instruction {
    let splitted: Vec<&str> = input.split(" ").collect();

    let command = match splitted[0] {
        "turn" => get_turn(splitted[1]),
        _ => Command::Toggle
    };

    let (from_x, from_y) = get_coords(splitted.get(splitted.len() - 3).unwrap());
    let (to_x, to_y) = get_coords(splitted.last().unwrap());

    return Instruction {
        command,
        from_x,
        from_y,
        to_x,
        to_y
    }
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();

    let mut lights: HashMap<(i32, i32), i32> = HashMap::new();

    for line in input.lines() {
        let instructions = get_instuction(line);

        for x in instructions.from_x..=instructions.to_x {
            for y in instructions.from_y..=instructions.to_y {
                match instructions.command {
                    Command::On => lights.entry((x,y)).and_modify(|l| {*l += 1}).or_insert(1),
                    Command::Off => lights.entry((x,y)).and_modify(|l| {
                        if *l <= 1 {
                            *l = 0
                        } else {
                            *l -= 1
                        }
                    }).or_insert(0),
                    Command::Toggle => lights.entry((x, y)).and_modify(|l| { *l += 2 }).or_insert(2),
                };
            }
        }
    }

    let sum: i32 = lights.into_values().sum();

    println!("{sum}")
}
