// AOC 2020 day 16

use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Ticket {
    nums: Vec<u32>,
}

impl Ticket {
    fn new(s: &str) -> Self {
        let ev = s.split(',').collect::<Vec<_>>();
        Self {
            nums: ev.iter().map(|s| s.parse::<u32>().unwrap()).collect(),
        }
    }
}

#[derive(Debug)]
struct Range {
    name: String,
    lower1: u32,
    upper1: u32,
    lower2: u32,
    upper2: u32,
}

impl Range {
    fn new(n: &str, s1: &str, s2: &str) -> Self {
        let ev1 = s1.split('-').collect::<Vec<_>>();
        let ev2 = s2.split('-').collect::<Vec<_>>();
        Self {
            name: n.to_string(),
            lower1: ev1[0].parse::<u32>().expect("bad ticket field"),
            upper1: ev1[1].parse::<u32>().expect("bad ticket field"),
            lower2: ev2[0].parse::<u32>().expect("bad ticket field"),
            upper2: ev2[1].parse::<u32>().expect("bad ticket field"),
        }
    }

    fn in_range(&self, n: u32) -> bool {
        (n >= self.lower1 && n <= self.upper1) || (n >= self.lower2 && n <= self.upper2)
    }
}

fn create_field_to_range_map(tix: &[Ticket], rngs: &[Range]) -> Vec<usize> {
    let mut ordered_poss_rng: Vec<(Vec<usize>, usize)> = vec![];

    for tidx in 0..tix[0].nums.len() {
        // build a vector of possible ranges for each ticket field
        let mut poss_rng: Vec<usize> = (0..rngs.len()).collect();

        for t in tix {
            let mut remove_vec: Vec<usize> = vec![];
            for gidx in 0..poss_rng.len() {
                if !rngs[poss_rng[gidx]].in_range(t.nums[tidx]) {
                    remove_vec.push(gidx);
                }
            }
            remove_vec.reverse();
            for i in remove_vec {
                poss_rng.remove(i);
            }
        }

        // keep track of the ticket field that created each possibilities vector
        ordered_poss_rng.push((poss_rng, tidx));
    }

    // experimentation shows that ordering the vector of good ranges by length allows
    // earlier fields to always constrain later;  In general this isn't necessarily true, but
    // here we will just check that it is so as we build the retval vector
    ordered_poss_rng.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());

    let mut retval: Vec<usize> = vec![usize::MAX; rngs.len()];
    for gv in ordered_poss_rng.iter_mut() {
        // remove already used ranges
        gv.0.retain(|elem| !retval.contains(elem));

        if gv.0.is_empty() {
            panic!("ticket field has no valid range");
        } else if gv.0.len() > 1 {
            panic!(
                "warning: ticket field has more than one valid range: {:?}",
                gv.0
            );
        } else {
            retval[gv.1] = gv.0[0];
        }
    }

    let test_retval: HashSet<usize> = retval.iter().cloned().collect();
    if test_retval.len() != rngs.len() {
        panic!("ticket field map does not cover all ranges: {:?}", retval);
    }

    retval
}

fn sum_and_remove_bad_values(tix: &mut Vec<Ticket>, rngs: &[Range]) -> u32 {
    let mut sum = 0;
    let mut remove_vec: Vec<usize> = vec![];
    for (i, t) in tix.iter().enumerate() {
        for n in &t.nums {
            if !rngs.iter().any(|r| r.in_range(*n)) {
                sum += n;
                remove_vec.push(i);
            }
        }
    }

    // remove bad tickets (note that remove_vec already in sorted order)
    remove_vec.reverse();
    for i in remove_vec {
        tix.remove(i);
    }

    sum
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let re_tix_fields = Regex::new(r"^([[:alpha:] ]+): (\d+-\d+) or (\d+-\d+)").unwrap();
    let mut ranges: Vec<Range> = vec![];
    let mut my_ticket = Ticket { nums: vec![] };
    let mut getting_nearby_tix = false;
    let mut nearby_tix: Vec<Ticket> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        if getting_nearby_tix {
            nearby_tix.push(Ticket::new(inputstr.trim()));
        } else if re_tix_fields.is_match(&inputstr) {
            let caps = re_tix_fields.captures(&inputstr).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str());
            let r1 = caps.get(2).map_or("", |m| m.as_str());
            let r2 = caps.get(3).map_or("", |m| m.as_str());
            ranges.push(Range::new(name, r1, r2));
        } else if inputstr.trim() == "your ticket:" {
            inputstr.clear();
            reader.read_line(&mut inputstr)?;
            my_ticket = Ticket::new(inputstr.trim());
        } else if inputstr.trim() == "nearby tickets:" {
            getting_nearby_tix = true;
        }

        inputstr.clear();
    }

    println!(
        "aoc16a: {} ",
        sum_and_remove_bad_values(&mut nearby_tix, &ranges)
    );

    // part 2
    let field_map = create_field_to_range_map(&nearby_tix, &ranges);

    // calculate the answer
    let mut ans: u64 = 1;
    for i in 0..my_ticket.nums.len() {
        if ranges[field_map[i]].name.split(' ').next().unwrap() == "departure" {
            ans *= my_ticket.nums[i] as u64;
        }
    }

    println!("aoc16b: {} ", ans);

    Ok(())
}
