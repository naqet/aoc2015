use std::fs;
fn calculate_ribbon(input: &str) -> i32 {
    let mut total_ribbon = 0;

    for line in input.lines() {
        let dimensions: Vec<&str> = line.split("x").collect();
        let l: i32 = dimensions[0].parse().unwrap();
        let w: i32 = dimensions[1].parse().unwrap();
        let h: i32 = dimensions[2].parse().unwrap();

        let bow = l * w * h;

        let mut list = vec![l, w, h];

        list.sort();

        total_ribbon += bow + (2 * list[0] + 2 * list[1]);
    }

    return total_ribbon;
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    println!("{:?}", calculate_ribbon(&input));
}
