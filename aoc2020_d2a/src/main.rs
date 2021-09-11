// AOC 2020 day 2

use regex::Regex;

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let re = Regex::new(r"^(\d+)-(\d+) (\w): ([[:alpha:]]+)").unwrap();
    let mut ans_a = 0;
    let mut ans_b = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if re.is_match(&inputstr) {
            let caps = re.captures(&inputstr).unwrap();
            let min = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let max = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let ch = caps.get(3).map_or("bad char", |m| m.as_str());
            let c = ch.chars().next().unwrap();
            let pw = caps.get(4).map_or("bad pw", |m| m.as_str());

            let count: usize = pw.chars().map(|x| if x == c { 1 } else { 0 }).sum();
            if count <= max && count >= min {
                ans_a += 1;
            }

            let mut found: bool = false;
            for (i, t) in pw.chars().enumerate() {
                if i + 1 == min && t == c {
                    found = true;
                } else if i + 1 == max {
                    if t == c && !found || t != c && found {
                        ans_b += 1;
                    }
                    break;
                }
            }
        }

        inputstr.clear();
    }

    println!("aoc2a: {}, aoc2b: {}", ans_a, ans_b);

    Ok(())
}
