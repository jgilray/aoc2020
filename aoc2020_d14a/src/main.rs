// AOC 2020 day 14

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
    val: char,
    loc: u8,
}

// create 2^n addresses from the X's in the mask
fn build_addresses(xlocs: &[u8], baseloc: u64) -> Vec<u64> {
    let two_to_n = 2_usize.pow(xlocs.len() as u32);
    let mut retval = vec![baseloc; two_to_n];

    // use 0..2^n to generate all permutations of 1s and 0s to insert for the X's
    for (i, adr) in retval.iter_mut().enumerate() {
        let mut val = i;
        for loc in xlocs {
            let v = val % 2;
            val /= 2;
            let mask = 1 << loc;
            if v == 0 {
                *adr &= !mask;
            } else {
                *adr |= mask;
            }
        }
    }

    retval
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let re_mask = Regex::new(r"^mask = ([[:alpha:]0-1]+)").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut vmask: Vec<Mask> = vec![];
    let mut mema: HashMap<u64, u64> = HashMap::new();
    let mut memb: HashMap<u64, u64> = HashMap::new();

    while reader.read_line(&mut inputstr)? != 0 {
        if re_mask.is_match(&inputstr) {
            let caps = re_mask.captures(&inputstr).unwrap();
            let maskstr = caps.get(1).map_or("", |m| m.as_str());
            vmask.clear();
            for (i, c) in maskstr.chars().enumerate() {
                match c {
                    '0' | '1' | 'X' => vmask.push(Mask {
                        val: c,
                        loc: 35 - i as u8,
                    }),
                    _ => panic!("bad mask"),
                }
            }
        } else if re_mem.is_match(&inputstr) {
            let caps = re_mem.captures(&inputstr).unwrap();
            let mut memloc = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<u64>().unwrap());
            let memval = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<u64>().unwrap());
            let mut memvala = memval;

            // set memory for problem 1
            for m in &vmask {
                let mask = 1 << m.loc;
                if m.val == '0' {
                    memvala &= !mask;
                } else if m.val == '1' {
                    memvala |= mask;
                }
            }
            mema.insert(memloc, memvala);

            // set memory for problem 2
            let mut xlocs: Vec<u8> = vec![];
            for m in &vmask {
                let mask = 1 << m.loc;
                if m.val == '1' {
                    memloc |= mask;
                } else if m.val == 'X' {
                    xlocs.push(m.loc);
                }
            }
            let mlocs = build_addresses(&xlocs, memloc);
            for l in &mlocs {
                memb.insert(*l, memval);
            }
        } else {
            panic!("bad input line");
        }

        inputstr.clear();
    }

    println!("aoc14a: {}", mema.values().sum::<u64>());
    println!("aoc14b: {}", memb.values().sum::<u64>());

    Ok(())
}
