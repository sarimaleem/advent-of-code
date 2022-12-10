use std::{collections::HashSet, fs};

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("{}", challenge1(input));
}

fn challenge1(input: String) -> i32 {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(|line| {
            let mut first = HashSet::new();
            for n in 0..line.len() / 2 {
                first.insert(line[n]);
            }

            for n in line.len() / 2..line.len() {
                let c = line[n];
                if first.contains(&c) {
                    return c;
                }
            }

            panic!("Overlapping not found");
        })
        .map(|letter| match letter {
            65..=90 => letter - 65 + 27,
            97..=122 => letter - 97 + 1,
            _ => panic!("Invalid character"),
        })
        .map(|letter| letter as i32)
        .sum()
}

fn challenge2(input: String) -> i32 {
    panic!("Not implemented");
}
