// AOC 2020 day 3

// function that traverses the passed forest f at that passed "angle" r (right) and d (down)
// returning the number of trees struck
fn count_tree_hits(f: &[Vec<char>], right: usize, down: usize) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut hits = 0;
    let rowlen = f[0].len();
    while x < f.len() {
        if f[x][y] == '#' {
            hits += 1;
        }
        x += down;
        y += right;
        y %= rowlen;
    }

    hits
}

fn main() {
    let input = include_str!("../../forest.dat");
    // a vector of vector of chars which represents the forest
    let forest_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut hits = count_tree_hits(&forest_vec, 3, 1);
    println!("aoc3a: {}", hits);

    hits *= count_tree_hits(&forest_vec, 1, 1);
    hits *= count_tree_hits(&forest_vec, 5, 1);
    hits *= count_tree_hits(&forest_vec, 7, 1);
    hits *= count_tree_hits(&forest_vec, 1, 2);
    println!("aoc3b: {}", hits);
}
