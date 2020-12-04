const INPUT: &str = include_str!("input");
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
const RESULT: u32 = SLOPES.iter().map(|slope| slide(INPUT, slope)).product();

fn main() {
    println!("{:?}", RESULT);
}

const fn slide(my_str: &str, slope: &(usize, usize)) -> u32 {
    let mut col = 0;
    let mut trees = 0;
    for line in my_str.lines().step_by(slope.1) {
        trees += has_tree(line, col);
        col += slope.0;
    }
    trees
}

const fn has_tree(line: &str, col: usize) -> u32 {
    (line.chars().nth(col % line.len()).unwrap() == '#') as u32
}
