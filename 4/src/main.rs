use md5;

fn main() {
    let mut random_number = 1;
    let input = "ckczppom";
    let mut hash = format!("{:?}", md5::compute(format!("{}{}", input, random_number)));

    while !hash.starts_with("000000") {
        random_number += 1;
        hash = format!("{:?}", md5::compute(format!("{}{}", input, random_number)));
    }
    println!("{random_number}");
}
