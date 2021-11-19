// AOC 2020 day 24

use std::collections::HashSet;

type HSet = HashSet<(isize, isize)>;

// function to update hexes according to the rules.  Can recurse once if first_call = true
fn update(x: isize, y: isize, hs: &HSet, nhs: &mut HSet, first_call: bool) {
    let offset = [(-2, 0), (-1, 1), (1, 1), (2, 0), (1, -1), (-1, -1)];
    let mut num_black_neighbors = 0;

    for (xo, yo) in offset {
        if hs.contains(&(x + xo, y + yo)) {
            num_black_neighbors += 1;
        } else if first_call {
            update(x + xo, y + yo, hs, nhs, false);
        }
    }

    if first_call && num_black_neighbors > 0 && num_black_neighbors < 3
        || !first_call && num_black_neighbors == 2
    {
        nhs.insert((x, y));
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut hs: HSet = HashSet::new();

    while reader.read_line(&mut inputstr)? != 0 {
        let mut citer = inputstr.trim().chars();

        // start at the center hexagon
        let mut x: isize = 0;
        let mut y: isize = 0;

        while let Some(c) = citer.next() {
            match c {
                'n' => {
                    y += 1;
                    let cc = citer.next().expect("missing sub-char");
                    match cc {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => panic!("bad direction sub-char: {}", cc),
                    }
                }
                's' => {
                    y -= 1;
                    let cc = citer.next().expect("missing sub-char");
                    match cc {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => panic!("bad direction sub-char: {}", cc),
                    }
                }
                'e' => x += 2,
                'w' => x -= 2,
                _ => panic!("bad direction char: {}", c),
            }
        }

        if hs.contains(&(x, y)) {
            hs.remove(&(x, y));
        } else {
            hs.insert((x, y));
        }

        inputstr.clear();
    }

    println!("aoc24a: {}", hs.len());

    // part two
    let mut nhs: HSet = HashSet::new();
    for _ in 1..=100 {
        for (x, y) in hs.iter() {
            update(*x, *y, &hs, &mut nhs, true);
        }

        // the following is slightly faster than declaring nhs in the loop and using "hs = nhs;"
        hs = nhs.clone();
        nhs.clear();
    }
    println!("aoc24b: {}", hs.len());

    Ok(())
}
