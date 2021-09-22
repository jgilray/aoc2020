// AOC 2020 day 15

use std::collections::HashMap;

// Play the elves game
fn play(
    hm: &mut HashMap<u32, (u32, u32)>,
    last_input: u32,
    first_turn: u32,
    last_turn: u32,
) -> u32 {
    let mut last_spoken = last_input;

    for turn in first_turn + 1..=last_turn {
        let (_, gap) = hm.get(&last_spoken).unwrap();
        if *gap == 0 {
            last_spoken = 0;
        } else {
            last_spoken = *gap;
        }

        let mut new_gap = 0;
        if hm.contains_key(&last_spoken) {
            let (prev, _) = hm.get(&last_spoken).unwrap();
            new_gap = turn - prev;
        }
        hm.insert(last_spoken, (turn, new_gap));
    }
    last_spoken
}

fn main() -> std::io::Result<()> {
    let vinput: Vec<u32> = vec![0, 6, 1, 7, 2, 19, 20];

    // the game space: key = number spoken, value = (turn#, gap between last two times spoken)
    // gap = 0 if first time spoken
    let mut hm: HashMap<u32, (u32, u32)> = HashMap::new();
    let last_input: u32 = *vinput.last().expect("bad vinput vec");

    // initialise hm with the input
    for (i, n) in vinput.iter().enumerate() {
        hm.insert(*n, (i as u32 + 1, 0));
    }
    println!(
        "aoc15a: {}",
        play(&mut hm, last_input, vinput.len() as u32, 2020)
    );

    // reinitialize for part 2
    hm.clear();
    for (i, n) in vinput.iter().enumerate() {
        hm.insert(*n, (i as u32 + 1, 0));
    }
    println!(
        "aoc15b: {}",
        play(&mut hm, last_input, vinput.len() as u32, 30_000_000)
    );

    Ok(())
}
