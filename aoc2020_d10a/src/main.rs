// AOC 2020 day 10

use std::collections::{HashMap, HashSet};

// scan the sorted joltages returning a tuple of (1-diffs, 3-diffs)
fn count_joltage_differences(v: &[usize]) -> (usize, usize) {
    let mut cur = 0;
    let mut diff1 = 0;
    let mut diff3 = 0;
    for i in v {
        match i - cur {
            1 => diff1 += 1,
            2 => {}
            3 => diff3 += 1,
            _ => break,
        }
        cur = *i;
    }

    (diff1, diff3)
}

fn count_arrangements(cur: usize, hs: &HashSet<usize>, hm: &mut HashMap<usize, usize>) -> usize {
    if let Some(count) = hm.get(&cur) {
        return *count;
    }

    let mut count = 0;
    let mut no_hits = true;
    if hs.contains(&(cur + 1)) {
        count += count_arrangements(cur + 1, hs, hm);
        no_hits = false;
    }
    if hs.contains(&(cur + 2)) {
        count += count_arrangements(cur + 2, hs, hm);
        no_hits = false;
    }
    if hs.contains(&(cur + 3)) {
        count += count_arrangements(cur + 3, hs, hm);
        no_hits = false;
    }

    if no_hits {
        count = 1;
    }

    hm.insert(cur, count);
    count
}

fn main() {
    let input = include_str!("../../joltages.dat");
    // ivec is a vector of numbers
    let mut ivec = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ivec.sort_unstable();
    let (d1, d3) = count_joltage_differences(&ivec);
    println!("aoc10a: {}", d1 * (d3 + 1));

    let hs: HashSet<usize> = ivec.iter().copied().collect::<_>();
    let mut hm: HashMap<usize, usize> = HashMap::new();
    println!("aoc10b: {}", count_arrangements(0, &hs, &mut hm));
}
