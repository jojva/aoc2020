fn main() {
    let input = include_str!("input");
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: u32 = slopes.iter().map(|slope| slide(input, slope)).product();
    println!("{:?}", result);
}

fn slide(input: &str, slope: &(usize, usize)) -> u32 {
    let mut col = 0;
    let mut trees = 0;
    for line in input.lines().step_by(slope.1) {
        trees += has_tree(line, col);
        col += slope.0;
    }
    trees
}

fn has_tree(line: &str, col: usize) -> u32 {
    (line.chars().nth(col % line.len()).unwrap() == '#') as u32
}
