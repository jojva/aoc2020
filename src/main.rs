fn main() {
    let my_str = include_str!("input");
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tree_map: i64 = slopes.iter().map(|slope| slide(my_str, slope)).product();
    println!("{:?}", tree_map);
}

fn slide(my_str: &str, slope: &(usize, usize)) -> i64 {
    let mut col = 0;
    let mut trees = 0;
    for line in my_str.lines().step_by(slope.1) {
        trees += has_tree(line, col);
        col += slope.0;
    }
    trees
}

fn has_tree(line: &str, col: usize) -> i64 {
    (line.chars().nth(col % line.len()).unwrap() == '#') as i64
}
