use std::{fs, collections::{HashMap, HashSet}};

struct Perms<T> {
    list: Vec<Vec<T>>
}

impl <T: Clone> Perms <T> {
    fn generate(&mut self, cities: &mut [T], size: usize) {
        if size == 1 {
            self.list.push(Vec::from(cities))
        } else {
            self.generate(cities, size - 1);

            for i in 0..size - 1 {
                if size % 2 == 0 {
                    cities.swap(i, size - 1);
                } else {
                    cities.swap(0, size - 1);
                }

                self.generate(cities, size - 1);
            }
        }
    }
    
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();

    let mut dists: HashMap<(String, String), u32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    for l in input.lines() {
        let info: Vec<&str> = l.split(" ").collect();

        let from = info[0];
        let to = info[2];
        let dist = info[4].parse::<u32>().unwrap();

        dists.insert((from.to_owned(), to.to_owned()), dist);
        dists.insert((to.to_owned(), from.to_owned()), dist);

        cities.insert(from.to_owned());
        cities.insert(to.to_owned());
    }

    let mut to_permutate = cities.iter().map(|s| s.to_owned()).collect::<Vec<String>>();
    let mut perms: Perms<String> = Perms { list: Vec::new() };

    perms.generate(&mut to_permutate, cities.len());

    let mut longest = 0;

    for p in perms.list {
        let mut total = 0;
        for pair in p.windows(2) {
            let to = &pair[0];
            let from = &pair[1];
            let dist = dists.get(&(to.to_owned(), from.to_owned())).unwrap();
            total += dist;
        }

        longest = longest.max(total);
    }

    println!("Distance: {}", longest);
}
