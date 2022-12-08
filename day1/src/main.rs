use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut totals = Vec::new();

    let mut cur_total = 0;
    for line in contents.lines() {
        if line.is_empty() {
            totals.push(cur_total);
            cur_total = 0;
        } else {
            cur_total +=  line.parse::<i32>().unwrap();
        }
    }

    totals.sort_by(|a, b| b.cmp(a));
    let max = totals[0];
    let first_three_sum: i32 = totals[0..3].iter().sum();
    println!("{} {}", max, first_three_sum)
}
