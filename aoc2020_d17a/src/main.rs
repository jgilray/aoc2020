// AOC 2020 day 17

use std::collections::{HashMap, HashSet};

// fills ngbr Vec with all neighbors of orig
fn create_neighbors(orig: &[u32], idx: usize, v: &mut Vec<u32>, ngbr: &mut Vec<Vec<u32>>) {
    if idx >= orig.len() {
        if v != orig {
            ngbr.push(v.clone());
        }
    } else {
        for i in orig[idx] - 1..=orig[idx] + 1 {
            v[idx] = i;
            create_neighbors(orig, idx + 1, v, ngbr);
        }
    }
}

// performs one cycle of the game of life (# dimensions determined by the length of the Vec)
fn cycle(f: &HashSet<Vec<u32>>) -> HashSet<Vec<u32>> {
    let mut f_next: HashSet<Vec<u32>> = HashSet::new();
    let mut inactives: HashMap<Vec<u32>, u32> = HashMap::new();

    // go through all active cubes looking at neighboring locations
    for active in f.iter() {
        let mut neighbors: u32 = 0;
        let mut ngbrs: Vec<Vec<u32>> = vec![];
        create_neighbors(active, 0, &mut active.clone(), &mut ngbrs);

        for nv in &ngbrs {
            if f.contains(nv) {
                neighbors += 1;
            } else {
                let counter = inactives.entry(nv.to_vec()).or_insert(0);
                *counter += 1;
            }
        }

        if neighbors == 2 || neighbors == 3 {
            f_next.insert(active.to_vec());
        }
    }

    // go through all inactive cubes found above to see which will become active
    for (loc, n) in inactives.iter() {
        if *n == 3 {
            f_next.insert(loc.to_vec());
        }
    }

    f_next
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut field3d: HashSet<Vec<u32>> = HashSet::new();
    let mut field4d: HashSet<Vec<u32>> = HashSet::new();
    let mut x = 10_u32;
    let mut y = 10_u32;

    while reader.read_line(&mut inputstr)? != 0 {
        for c in inputstr.trim_end().chars() {
            match c {
                '#' => {
                    field3d.insert([x, y, 10].to_vec());
                    field4d.insert([x, y, 10, 10].to_vec());
                    x += 1;
                }
                '.' => x += 1,
                _ => panic!("bad input char: {}", c),
            }
        }

        x = 10;
        y += 1;
        inputstr.clear();
    }

    // run 6 cycles, updating the field of play
    for _ in 0..6 {
        field3d = cycle(&field3d);
        field4d = cycle(&field4d);
    }

    println!("aoc16a: {}", field3d.len());
    println!("aoc16b: {}", field4d.len());

    Ok(())
}
