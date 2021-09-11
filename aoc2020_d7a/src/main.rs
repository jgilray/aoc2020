// AOC 2020 day 7

use std::collections::{HashSet, VecDeque};
type BagVec = Vec<(u32, String)>;

// recursively count the contents of the passed bag
fn count_contents(bag: &str, bv: &[BagVec]) -> u32 {
    let mut sum = 0;
    for con in bv.iter() {
        if con[0].1 != bag {
            continue;
        }
        for c in con.iter().skip(1) {
            sum += c.0 + c.0 * count_contents(&c.1, bv);
        }
        break;
    }

    sum
}

// search for all bags that could contain the passed bag, return a HashSet of their indices
fn bfs_containers(bag: &str, bv: &[BagVec]) -> HashSet<usize> {
    let mut fifo: VecDeque<&str> = VecDeque::new(); // for BFS
    let mut hash: HashSet<usize> = HashSet::new(); // container hits

    // add the starting bag to the fifo
    // also add its index to the hash - not strictly necessary in this case
    fifo.push_back(bag);
    hash.insert(
        bv.iter()
            .enumerate()
            .take_while(|(_, con)| con[0].1 != bag)
            .map(|(i, _)| i)
            .last()
            .unwrap(),
    );

    // search bags in breadth first search manner, looking for target
    while !fifo.is_empty() {
        let sbag = fifo.pop_front().unwrap();

        for (i, con) in bv.iter().enumerate() {
            for c in con.iter().skip(1) {
                if c.1 == sbag && !hash.contains(&i) {
                    hash.insert(i);
                    fifo.push_back(&con[0].1);
                }
            }
        }
    }

    hash
}

// take each bag line and turn it into a vector of tuples (num, name)
fn gather_bag_data(s: &str) -> BagVec {
    let mut retv: BagVec = vec![];
    let rawv = s.split_whitespace().collect::<Vec<_>>();

    if rawv.len() > 2 {
        retv.push((1, rawv[0].to_string() + rawv[1])); // container bag
    }

    if rawv.len() > 7 {
        let mut idx = 4;
        loop {
            let num_bags = rawv[idx].parse::<u32>().expect("invalid number of bags");
            retv.push((num_bags, rawv[idx + 1].to_string() + rawv[idx + 2])); // contained bag
            idx += 4;
            if idx > rawv.len() - 1 {
                break;
            }
        }
    }

    retv
}

fn main() {
    let input = include_str!("../../bag_rules.dat");
    let bvec = input
        .lines()
        .map(|line| gather_bag_data(line))
        .collect::<Vec<_>>();

    let hs = bfs_containers("shinygold", &bvec);
    println!("aoc7a: {}", hs.len() - 1); // subtract the starting bag
    println!("aoc7b: {}", count_contents("shinygold", &bvec));
}
