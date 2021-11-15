// AOC 2020 day 22 - VecDeque version

use std::collections::HashSet;
use std::collections::VecDeque;

// score a deck
fn score(deck: &VecDeque<usize>) -> usize {
    let mut mult = deck.len() + 1;
    deck.iter()
        .map(|c| {
            mult -= 1;
            c * mult
        })
        .sum()
}

// card game - recursive version when recurse is true, returns true if player1 wins
fn combat(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, recurse: bool) -> bool {
    let mut prev_decks: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    // combat until one player is out of cards
    while !p1.is_empty() && !p2.is_empty() {
        // note that Vecs are about twice as fast in HashSets compared to VecDeques
        let state: (Vec<usize>, Vec<usize>) =
            (p1.iter().copied().collect(), p2.iter().copied().collect());
        let p1first = p1.pop_front().unwrap();
        let p2first = p2.pop_front().unwrap();
        let p1_wins: bool = if recurse {
            if !prev_decks.insert(state) {
                true
            } else if p1.len() >= p1first && p2.len() >= p2first {
                let mut p1c: VecDeque<usize> = p1.iter().take(p1first).copied().collect();
                let mut p2c: VecDeque<usize> = p2.iter().take(p2first).copied().collect();

                combat(&mut p1c, &mut p2c, true)
            } else {
                p1first > p2first
            }
        } else {
            p1first > p2first
        };

        if p1_wins {
            p1.push_back(p1first);
            p1.push_back(p2first);
        } else {
            p2.push_back(p2first);
            p2.push_back(p1first);
        }
    }

    p2.is_empty()
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut p1_deck: VecDeque<usize> = VecDeque::new();
    let mut p2_deck: VecDeque<usize> = VecDeque::new();
    let mut deal_to_player1 = true;

    while reader.read_line(&mut inputstr)? != 0 {
        let instr = inputstr.trim().to_string();

        if instr == "Player 1:" {
            deal_to_player1 = true;
        } else if instr == "Player 2:" {
            deal_to_player1 = false;
        } else if !instr.is_empty() {
            let card = instr.parse::<usize>().unwrap();
            if deal_to_player1 {
                p1_deck.push_back(card);
            } else {
                p2_deck.push_back(card);
            }
        }

        inputstr.clear();
    }

    let mut p1_d = p1_deck.clone();
    let mut p2_d = p2_deck.clone();
    if combat(&mut p1_d, &mut p2_d, false) {
        println!("aoc22a: {}", score(&p1_d));
    } else {
        println!("aoc22a: {}", score(&p2_d));
    }

    if combat(&mut p1_deck, &mut p2_deck, true) {
        println!("aoc22b: {}", score(&p1_deck));
    } else {
        println!("aoc22b: {}", score(&p2_deck));
    }

    Ok(())
}
