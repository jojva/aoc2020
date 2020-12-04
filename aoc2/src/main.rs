fn main() {
    let input = include_str!("input");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut valid: u32 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(|c: char| c == '-' || c == ':' || c.is_whitespace())
            .collect();
        let min_occurrences: u32 = parts[0].parse().unwrap();
        let max_occurrences: u32 = parts[1].parse().unwrap();
        let letter_to_check: char = parts[2].parse().unwrap();
        let password: &str = parts[4];
        valid +=
            is_password_valid(password, letter_to_check, min_occurrences, max_occurrences) as u32;
    }
    println!("{}", valid);
}

fn is_password_valid(
    password: &str,
    letter_to_check: char,
    min_occurrences: u32,
    max_occurrences: u32,
) -> bool {
    let letters: u32 = password
        .chars()
        .map(|c| (c == letter_to_check) as u32)
        .sum();
    (min_occurrences..=max_occurrences).contains(&letters)
}

fn part2(input: &str) {
    let mut valid: u32 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(|c: char| c == '-' || c == ':' || c.is_whitespace())
            .collect();
        let pos_a: usize = parts[0].parse().unwrap();
        let pos_b: usize = parts[1].parse().unwrap();
        let letter_to_check: char = parts[2].parse().unwrap();
        let password: &str = parts[4];
        valid += is_password_valid_2(password, letter_to_check, pos_a, pos_b) as u32;
    }
    println!("{}", valid);
}

fn is_password_valid_2(password: &str, letter_to_check: char, pos_a: usize, pos_b: usize) -> bool {
    let mut found = 0;
    for pos in [pos_a, pos_b].iter() {
        if password.chars().nth(*pos - 1).unwrap() == letter_to_check {
            found += 1;
        }
    }
    found == 1
}
