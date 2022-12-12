use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("{}", challenge1(&input));
    println!("{}", challenge2(&input));
}

fn challenge1(input: &String) -> i32 {
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

fn challenge2(input: &String) -> u32 {
    input
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .map(|(l1, l2, l3)| {
            let a: HashSet<u8> = HashSet::from_iter(l1.iter().cloned());
            let b: HashSet<u8> = HashSet::from_iter(l2.iter().cloned());
            let intersection: Vec<&u8> = l3
                .iter()
                .filter(|letter| a.contains(letter) && b.contains(letter))
                .collect();
            return **intersection.get(0).unwrap();
        })
        .map(|letter| match letter {
            b'A'..=b'Z' => letter - b'A' + 27,
            b'a'..=b'z' => letter - b'a' + 1,
            _ => panic!("Invalid character"),
        })
        .map(u32::from)
        .sum()
}
