use std::{fs::File, io::{BufReader, Read}, process};

fn calculate_line(s: &str) -> (usize, usize) {
    let mut escaped = 0;
    let mut index = 1;

    let chars = s.chars().collect::<Vec<char>>();

    while index < chars.len() - 1 {
        escaped += 1;
        if chars[index] == '\\' {
            if chars[index + 1] == 'x' {
                index += 4;
            } else {
                index += 2;
            }
        } else {
            index += 1;
        }
    }

    (s.len(), escaped)
}

fn encode_line(s: &str) -> (usize, usize) {
    let count = s
        .chars()
        .map(|c| if matches!(c, '"' | '\\') { 2 } else { 1 })
        .sum::<usize>();

    (s.len(), count + 2)
}

fn main() {
    let file = File::open("src/input").unwrap();

    let mut buf_reader = BufReader::new(file);

    let mut input = String::new();

    if let Err(e) = buf_reader.read_to_string(&mut input) {
        eprintln!("{e}");
        process::exit(1);
    }

    let mut total = 0;
    let mut encoded = 0;

    for line in input.lines() {
        let (t, e) = encode_line(line);
        total += t;
        encoded += e;
    }

    println!("{}", encoded - total);
}

#[cfg(test)]
mod tests {
    use super::{encode_line, calculate_line};

    #[test]
    fn count_empty_string() {
        assert_eq!((2, 0), calculate_line("\"\""));
    }

    #[test]
    fn count_string() {
        assert_eq!((5, 3), calculate_line("\"abc\""));
    }

    #[test]
    fn count_quote() {
        assert_eq!((10, 7), calculate_line("\"aaa\\\"aaa\""));
    }

    #[test]
    fn count_hex() {
        assert_eq!((6, 1), calculate_line("\"\\x27\""));
    }

    #[test]
    fn encode_empty_string() {
        assert_eq!((2, 6), encode_line("\"\""));
    }

    #[test]
    fn encode_string() {
        assert_eq!((5, 9), encode_line("\"abc\""));
    }

    #[test]
    fn encode_backslash() {
        assert_eq!((10, 16), encode_line("\"aaa\\\"aaa\""));
    }

    #[test]
    fn encode_hex() {
        assert_eq!((6, 11), encode_line("\"\\x27\""));
    }
}
