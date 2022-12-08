use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut totals = Vec::new();

    let mut cur_total = 0;
    for line in contents.lines() {
        if line == "" {
            totals.push(cur_total);
            cur_total = 0;
        } else {
            let num: i32 = line.parse().unwrap();
            cur_total += num;
        }
    }

    let max = totals.iter().max().unwrap();
    println!("{}", max);

    totals.sort_by(|a, b| b.cmp(a));
    let first_three_sum: i32 = totals[0..3].iter().sum();
    println!("{}",  first_three_sum)
}
