// AOC 2020 day 6

use std::collections::HashSet;

fn uniques(ans: &str) -> usize {
    let mut uhs: HashSet<char> = HashSet::new();
    for a in ans.split_whitespace() {
        for c in a.chars() {
            uhs.insert(c);
        }
    }

    uhs.len()
}

fn commons(ans: &str) -> usize {
    let mut cv: Vec<usize> = vec![0; 26];

    let mut group_size = 0;
    for a in ans.split_whitespace() {
        for c in a.chars() {
            cv[c as usize - b'a' as usize] += 1;
        }
        group_size += 1;
    }

    cv.iter().map(|n| if *n < group_size { 0 } else { 1 }).sum()
}

fn main() {
    let input = include_str!("../../customs_answers.dat");
    let vec = input.split("\n\n").collect::<Vec<_>>();

    let total: usize = vec.iter().map(|t| uniques(t)).sum();
    println!("aoc6a: {}", total);

    let total: usize = vec.iter().map(|t| commons(t)).sum();
    println!("aoc6b: {}", total);
}
