use std::{collections::HashSet, fs, time::Instant};

fn is_sum_of_two_previous_numbers(target: i128, numbers: &[i128]) -> bool {
    let mut seen = HashSet::with_capacity(numbers.len());
    for number in numbers {
        if seen.contains(&(target - number)) {
            return true;
        }
        seen.insert(number);
    }

    false
}

fn solve(numbers: &[i128], n: usize) -> Vec<i128> {
    let mut numbers_that_crumble = vec![];
    let mut i = numbers.len();

    while n < i {
        let previous_numbers = &numbers[i - (n + 1)..i - 1];
        let target = numbers[i - 1];

        if !is_sum_of_two_previous_numbers(target, previous_numbers) {
            numbers_that_crumble.push(target);
        }

        i -= 1;
    }

    numbers_that_crumble
}

fn main() {
    let numbers: Vec<i128> = fs::read_to_string("challenge_input.txt")
        .expect("File not found!")
        .lines()
        .map(|n| n.parse().expect("Failed converting number to usize"))
        .collect();

    let t1 = Instant::now();
    let numbers_that_crumble = solve(&numbers, 100);
    let t2 = Instant::now();

    println!("{:?}", numbers_that_crumble);
    println!("TOTAL TIME: {:?}", t2 - t1);
}
