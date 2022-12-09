use std::fs;


enum Selection {
    Rock,
    Paper,
    Scissors
}

enum RoundResult {
    Win,
    Loss,
    Draw
}

fn main() {
    println!("{} {}", challenge1(), challenge2());
}

fn challenge2() -> u32 {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut total: u32 = 0;
    for line in contents.lines() {
        let mut iter = line.split_ascii_whitespace();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        let choice1 = match first {
            "A" => Selection::Rock,
            "B" => Selection::Paper,
            "C" => Selection::Scissors,
            _ => panic!("invalid")
        };

        let result = match second {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("invalid")
        };

        total += match result {
            RoundResult::Win => 6,
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
        };

        let choice2 = get_choice(choice1, result);
        total += match choice2 {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        };
    }
    return total;
}

fn challenge1() -> u32 {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut total: u32 = 0;
    for line in contents.lines() {
        let mut iter = line.split_ascii_whitespace();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        let choice1 = match first {
            "A" => Selection::Rock,
            "B" => Selection::Paper,
            "C" => Selection::Scissors,
            _ => panic!("invalid")
        };

        let choice2 = match second {
            "X" => Selection::Rock,
            "Y" => Selection::Paper,
            "Z" => Selection::Scissors,
            _ => panic!("invalid")
        };

        total += match choice2 {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        };

        total += get_points(choice2, choice1);
    }

    return total;
}

fn get_points(choice1: Selection, choice2: Selection) -> u32 {
    return match choice1 {
       Selection::Rock => match choice2 {
        Selection::Rock => 3,
        Selection::Paper => 0,
        Selection::Scissors => 6
       },
       Selection::Paper => match choice2 {
        Selection::Rock => 6,
        Selection::Paper => 3,
        Selection::Scissors => 0
       },
       Selection::Scissors => match choice2 {
        Selection::Rock => 0,
        Selection::Paper => 6,
        Selection::Scissors => 3
       },
    };
}

fn get_choice(choice1: Selection, choice2: RoundResult) -> Selection {
    return match choice1 {
       Selection::Rock => match choice2 {
        RoundResult::Win => Selection::Paper,
        RoundResult::Loss => Selection::Scissors,
        RoundResult::Draw => Selection::Rock
       },
       Selection::Paper => match choice2 {
        RoundResult::Win => Selection::Scissors,
        RoundResult::Loss => Selection::Rock,
        RoundResult::Draw => Selection::Paper
       },
       Selection::Scissors => match choice2 {
        RoundResult::Win => Selection::Rock,
        RoundResult::Loss => Selection::Paper,
        RoundResult::Draw => Selection::Scissors
       }
    };
}
