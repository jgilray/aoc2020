// AOC 2020 day 23

use std::collections::VecDeque;

// perform a single move as described in the game
// with the modification that v is always rotated so that the next "current cup" is at the front
fn onemove(v: &mut VecDeque<usize>, radix: usize) {
    let first = v.pop_front().unwrap();
    let tmp: Vec<usize> = v.iter().take(3).copied().collect();
    v.pop_front();
    v.pop_front();
    v.pop_front();

    for sub in 2..6 {
        let goal = (first + radix - sub) % radix + 1;
        if v.contains(&goal) {
            for idx in 0..v.len() {
                if v[idx] == goal {
                    for (tmp_idx, tmp_item) in tmp.iter().enumerate().take(3) {
                        v.insert(idx + tmp_idx + 1, *tmp_item);
                    }
                    break;
                }
            }

            break;
        }
    }
    v.push_back(first);
}

fn main() -> std::io::Result<()> {
    // to do example set instr to "389125467"
    let instr: String = "157623984".to_string();
    let mut v: VecDeque<usize> = instr.chars().map(|c| c as usize - b'0' as usize).collect();

    // part one - simple approach using a VecDeque
    for _ in 0..100 {
        onemove(&mut v, 9);
    }

    // rotate until 1 is at front
    while v[0] != 1 {
        v.rotate_left(1);
    }
    let ans = v.iter().skip(1).fold(0, |s, i| 10 * s + i);
    println!("aoc23a: {}", ans);

    // part two - brute force approach above didn't work.  Using an array of "next" pointers
    v = instr.chars().map(|c| c as usize - b'0' as usize).collect();
    v.extend(10..=1_000_000);

    let mut next_cup = vec![0; v.len() + 1];
    for i in 0..v.len() - 1 {
        next_cup[v[i]] = v[i + 1];
    }
    next_cup[v[v.len() - 1]] = v[0];

    let mut current = v[0];
    for _ in 0..10_000_000 {
        let mut to_move = vec![];
        let mut tmp = current;
        for _ in 0..3 {
            tmp = next_cup[tmp];
            to_move.push(tmp);
        }

        for sub in 2..6 {
            let goal = (current + v.len() - sub) % v.len() + 1;
            if !to_move.contains(&goal) {
                next_cup.swap(goal, current);
                next_cup.swap(current, to_move[2]);
                break;
            }
        }
        current = next_cup[current];
    }
    println!("aoc23b: {}", next_cup[1] * next_cup[next_cup[1]]);

    Ok(())
}
