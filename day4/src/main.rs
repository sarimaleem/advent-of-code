use itertools::Itertools;
use std::cmp;
use std::fs;

fn main() {
    println!("Hello, world!");
    let path = "./input.txt";
    let input = fs::read_to_string(path).unwrap();
    println!("{} {}", challenge1(&input), challenge2(&input));
}

fn challenge1(input: &String) -> u32 {
    input
        .lines()
        .flat_map(|line| line.split(",").collect::<Vec<&str>>())
        .flat_map(|line| line.split("-").collect::<Vec<&str>>())
        .map(|l| l.parse::<u32>().unwrap())
        .tuples()
        .map(|(r1_start, r1_end, r2_start, r2_end)| {
            r1_start >= r2_start && r1_end <= r2_end || r2_start >= r1_start && r2_end <= r1_end
        })
        .map(u32::from)
        .sum()
}

fn challenge2(input: &String) -> u32 {
    input
        .lines()
        .flat_map(|line| line.split(",").collect::<Vec<&str>>())
        .flat_map(|line| line.split("-").collect::<Vec<&str>>())
        .map(|l| l.parse::<u32>().unwrap())
        .tuples()
        .map(|(r1_start, r1_end, r2_start, r2_end)| {
            cmp::max(r1_start, r2_start) <= cmp::min(r1_end, r2_end)
        })
        .map(u32::from)
        .sum()
}
