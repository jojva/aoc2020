use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut entries = HashSet::new();
    for line in input.lines() {
        let entry = line.parse::<u32>().unwrap();
        if entries.contains(&(2020 - entry)) {
            println!("{}", entry * (2020 - entry));
            break;
        }
        entries.insert(entry);
    }
}

fn part2(input: &str) {
    let entries: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    for i in entries.iter() {
        for j in entries.iter() {
            for k in entries.iter() {
                if i + j + k == 2020 {
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                }
            }
        }
    }
}
