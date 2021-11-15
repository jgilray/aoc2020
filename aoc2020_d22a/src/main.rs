// AOC 2020 day 22

use std::collections::HashSet;

// score a deck
fn score(deck: &[usize]) -> usize {
    let mut mult = deck.len() + 1;
    deck.iter()
        .map(|c| {
            mult -= 1;
            c * mult
        })
        .sum()
}

// card game - recursive version when recurse is true, returns true if player1 wins
fn combat(p1: &mut Vec<usize>, p2: &mut Vec<usize>, recurse: bool) -> bool {
    let mut prev_decks: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    // combat until one player is out of cards
    while !p1.is_empty() && !p2.is_empty() {
        let p1_wins: bool = if recurse {
            let state: (Vec<usize>, Vec<usize>) =
                (p1.iter().copied().collect(), p2.iter().copied().collect());
            if !prev_decks.insert(state) {
                true
            } else if p1.len() > p1[0] && p2.len() > p2[0] {
                let mut p1c: Vec<usize> = p1[1..=p1[0]].iter().copied().collect();
                let mut p2c: Vec<usize> = p2[1..=p2[0]].iter().copied().collect();

                combat(&mut p1c, &mut p2c, true)
            } else {
                p1[0] > p2[0]
            }
        } else {
            p1[0] > p2[0]
        };

        if p1_wins {
            p1.push(p1[0]);
            p1.push(p2[0]);
        } else {
            p2.push(p2[0]);
            p2.push(p1[0]);
        }
        p1.remove(0);
        p2.remove(0);
    }

    p2.is_empty()
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut p1_deck: Vec<usize> = vec![];
    let mut p2_deck: Vec<usize> = vec![];
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
                p1_deck.push(card);
            } else {
                p2_deck.push(card);
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
        println!("aoc22a: {}", score(&p1_deck));
    } else {
        println!("aoc22a: {}", score(&p2_deck));
    }

    Ok(())
}
