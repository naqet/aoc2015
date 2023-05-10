fn look_and_say(input: &str) -> String {
    let mut new = String::new();
    let mut last_char = ' ';
    let mut count = 1;

    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            last_char = c;
        } else if i == input.len() - 1 {
            
            if last_char == c {
                count += 1;
                new.push(char::from_digit(count, 10).unwrap());
                new.push(c);
            } else {
                new.push(char::from_digit(count, 10).unwrap());
                new.push(last_char);
                new.push('1');
                new.push(c);
            }

        } else if last_char == c {
            count += 1;
            last_char = c;
        } else {
            // Pushing to new string
            new.push(char::from_digit(count, 10).unwrap());
            new.push(last_char);

            last_char = c;
            count = 1;
        };
    };

    new
}

fn main() {
    let input = "1321131112";
    let mut result = look_and_say(input);

    for _ in 0..49 {
        result = look_and_say(&mut result);
    }

    println!("{:?}", result.len());

}
